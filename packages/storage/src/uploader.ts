export interface UploadOptions {
  method?: "POST" | "PUT";
  headers?: Record<string, string>;
  fieldName?: string; // for multipart POST uploads
  extraFields?: Record<string, string>; // for multipart uploads
  onProgress?: (progress: number) => void;
}

export class OrbisStorageUploader {
  /**
   * Upload a file/buffer using standard XMLHttpRequest or fetch fallback for progress tracking.
   * Works across Web, Tauri, and Mobile.
   */
  static upload(
    url: string,
    fileData: File | Blob | ArrayBuffer | string, // can accept base64 string or file buffer
    options: UploadOptions = {},
  ): Promise<{ success: boolean; data: any }> {
    return new Promise((resolve, reject) => {
      const method = options.method || "POST";
      const headers = options.headers || {};
      const fieldName = options.fieldName || "file";
      const extraFields = options.extraFields || {};
      const onProgress = options.onProgress;

      // Check if XMLHttpRequest is available (Browser / React Native / Tauri)
      if (typeof XMLHttpRequest !== "undefined") {
        const xhr = new XMLHttpRequest();
        xhr.open(method, url, true);

        // Set custom headers
        for (const [key, value] of Object.entries(headers)) {
          xhr.setRequestHeader(key, value);
        }

        // Setup progress hook
        if (onProgress && xhr.upload) {
          xhr.upload.onprogress = (event) => {
            if (event.lengthComputable) {
              const progress = Math.round((event.loaded / event.total) * 100);
              onProgress(progress);
            }
          };
        }

        xhr.onload = () => {
          if (xhr.status >= 200 && xhr.status < 300) {
            let parsed: any;
            try {
              parsed = JSON.parse(xhr.responseText);
            } catch {
              parsed = xhr.responseText;
            }
            resolve({ success: true, data: parsed });
          } else {
            reject(
              new Error(
                `Upload failed with status: ${xhr.status} ${xhr.statusText}`,
              ),
            );
          }
        };

        xhr.onerror = () => {
          reject(new Error("Network error occurred during file upload."));
        };

        // Determine body format (Multipart Form vs Raw Binary)
        if (method === "POST" && !(fileData instanceof ArrayBuffer)) {
          // Use FormData for standard Multipart upload
          const formData = new FormData();

          // Append extra fields
          for (const [key, value] of Object.entries(extraFields)) {
            formData.append(key, value);
          }

          // Append file
          if (fileData instanceof File || fileData instanceof Blob) {
            formData.append(fieldName, fileData);
          } else if (
            typeof fileData === "string" &&
            fileData.startsWith("data:")
          ) {
            // Handle Base64 URL representation
            const parsedBlob = this.dataURItoBlob(fileData);
            formData.append(fieldName, parsedBlob, "upload_file");
          } else {
            formData.append(fieldName, fileData);
          }

          xhr.send(formData);
        } else {
          // Raw binary PUT upload (commonly used for direct S3 presigned URLs)
          let sendData: any = fileData;
          if (typeof fileData === "string" && fileData.startsWith("data:")) {
            sendData = this.dataURItoBlob(fileData);
          }
          xhr.send(sendData);
        }
      } else {
        // Fallback to standard fetch when running in pure runtime environments (Node, Deno, Edge)
        const fetchHeaders = new Headers(headers);
        let body: any;

        if (method === "POST") {
          const formData = new FormData();
          for (const [key, value] of Object.entries(extraFields)) {
            formData.append(key, value);
          }
          if (fileData instanceof Blob) {
            formData.append(fieldName, fileData);
          } else if (
            typeof fileData === "string" &&
            fileData.startsWith("data:")
          ) {
            formData.append(fieldName, this.dataURItoBlob(fileData));
          } else {
            const blob = new Blob([fileData as any]);
            formData.append(fieldName, blob);
          }
          body = formData;
        } else {
          body = fileData;
        }

        fetch(url, { method, headers: fetchHeaders, body })
          .then(async (response) => {
            if (response.ok) {
              let parsed: any;
              try {
                parsed = await response.json();
              } catch {
                parsed = await response.text();
              }
              if (onProgress) onProgress(100);
              resolve({ success: true, data: parsed });
            } else {
              reject(
                new Error(
                  `Upload failed with status: ${response.status} ${response.statusText}`,
                ),
              );
            }
          })
          .catch((err) => {
            reject(new Error(`Fetch upload failed: ${err.message}`));
          });
      }
    });
  }

  /**
   * Helper to convert base64 data URIs into binary Blobs.
   */
  private static dataURItoBlob(dataURI: string): Blob {
    const parts = dataURI.split(",");
    const byteString = atob(parts[1]!);
    const mimeString = parts[0]!.split(":")[1]!.split(";")[0]!;
    const ab = new ArrayBuffer(byteString.length);
    const ia = new Uint8Array(ab);
    for (let i = 0; i < byteString.length; i++) {
      ia[i] = byteString.charCodeAt(i);
    }
    return new Blob([ab], { type: mimeString });
  }
}
