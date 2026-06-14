"use client";

import { useEffect, useMemo, useRef, useState } from "react";

interface ScanLineAnimationProps {
  className?: string;
  isProcessing?: boolean;
  data?: {
    id?: string;
    model?: string;
    fileName?: string;
    timestamp?: string;
    status?: string;
  };
}

const VERTEX_SHADER_SRC = `
  attribute vec2 position;
  varying vec2 vUv;
  void main() {
    vUv = position * 0.5 + 0.5;
    gl_Position = vec4(position, 0.0, 1.0);
  }
`;

const FRAGMENT_SHADER_SRC = `
  precision highp float;
  uniform vec2 u_resolution;
  uniform float u_time;
  uniform float u_progress;
  uniform bool u_is_processing;
  varying vec2 vUv;

  // Pseudo-random generator based on UV coordinates
  float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453123);
  }

  void main() {
    vec2 st = vUv;
    
    // Base scanner grid cell dimensions (pixel quantization)
    float cols = 48.0;
    float rows = floor(cols * (u_resolution.y / u_resolution.x));
    if (rows <= 0.0) rows = 1.0;
    
    vec2 gridUv = vec2(floor(st.x * cols) / cols, floor(st.y * rows) / rows);
    
    vec3 baseColor = vec3(0.0);
    
    // Dynamic scan line progress (horizontal scanner line)
    float progress = u_progress;
    
    // Smooth wavy liquid distortion
    float waveOffset = sin(st.x * 12.0 + u_time * 3.5) * 0.008;
    
    float alpha = 0.0;
    
    if (u_is_processing) {
      float timeStep = floor(u_time * 12.0);
      float cellY = gridUv.y + waveOffset;
      
      // 1. Scanned Trail Region (Above the scan line: cellY > progress)
      if (cellY > progress) {
        float distanceScanned = cellY - progress;
        float activeTrailRange = 0.38; // Trailing scan height
        
        if (distanceScanned < activeTrailRange) {
          float randVal = random(gridUv + timeStep * 0.007);
          float intensity = 1.0 - (distanceScanned / activeTrailRange);
          
          if (randVal > (1.0 - intensity * 0.88)) {
            // Neon violet-indigo glow (rgb(167, 139, 250))
            vec3 pixelColor = vec3(0.65, 0.54, 0.98) * (randVal * 1.5 * intensity);
            baseColor = max(baseColor, pixelColor);
          }
          
          // Fine scanning line accents in the trail
          float lineNoise = random(vec2(gridUv.y, timeStep * 0.05));
          if (lineNoise > 0.96) {
            baseColor += vec3(0.54, 0.36, 0.96) * 0.35 * intensity;
          }
        }
        
        alpha = max(baseColor.r, max(baseColor.g, baseColor.b));
      }
      // 2. Unscanned Region (Below the scan line: cellY <= progress)
      else {
        // Pure solid dark color matching the dark-mode dashboard background (0, 0%, 5% = rgb(13,13,13) = 0.05)
        baseColor = vec3(0.05, 0.05, 0.05);
        alpha = 1.0;
      }
      
      // 3. Laser scan line directly at the progress boundary with wave distortion
      float laserLine = smoothstep(0.006, 0.0, abs((st.y + waveOffset) - progress));
      if (laserLine > 0.0) {
        vec3 laserGlow = vec3(0.75, 0.68, 1.0) * laserLine * 1.6;
        baseColor = max(baseColor, laserGlow);
        alpha = max(alpha, laserLine);
      }
    } else {
      // Completed scanning state: fully transparent to show image
      alpha = 0.0;
    }
    
    gl_FragColor = vec4(baseColor, alpha);
  }
`;

/**
 * High Performance WebGL Scanner Animation
 * Implements quantized grid cells and pixel-scatter transition shader
 * mimicking the mathematical visual effect of tympanus/Codrops WebGL transitions.
 */
export function ScanLineAnimation({
  className,
  isProcessing = true,
  data,
}: ScanLineAnimationProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [_randomId, setRandomId] = useState("");
  const [webglSupported, setWebglSupported] = useState(true);

  useEffect(() => {
    setRandomId(Math.random().toString(36).substring(7).toUpperCase());

    // Verify WebGL availability
    const testCanvas = document.createElement("canvas");
    const gl =
      testCanvas.getContext("webgl") ||
      testCanvas.getContext("experimental-webgl");
    if (!gl) {
      setWebglSupported(false);
    }
  }, []);

  // Parse progress percentage from status if present (e.g. "PROGRESS_60%")
  const parsedProgress = useMemo(() => {
    if (!data?.status) return -1;
    const match = data.status.match(/PROGRESS_(\d+)%/);
    if (match?.[1]) {
      return parseFloat(match[1]) / 100;
    }
    return -1;
  }, [data?.status]);

  useEffect(() => {
    if (!webglSupported) return;

    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;

    // Initialize WebGL
    const gl = canvas.getContext("webgl", { alpha: true });
    if (!gl) {
      setWebglSupported(false);
      return;
    }

    // Helper: Compile shader
    const compileShader = (source: string, type: number) => {
      const shader = gl.createShader(type);
      if (!shader) return null;
      gl.shaderSource(shader, source);
      gl.compileShader(shader);
      if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error("Shader compile error:", gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
        return null;
      }
      return shader;
    };

    const vs = compileShader(VERTEX_SHADER_SRC, gl.VERTEX_SHADER);
    const fs = compileShader(FRAGMENT_SHADER_SRC, gl.FRAGMENT_SHADER);
    if (!vs || !fs) return;

    const program = gl.createProgram();
    if (!program) return;
    gl.attachShader(program, vs);
    gl.attachShader(program, fs);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      console.error("Program link error:", gl.getProgramInfoLog(program));
      return;
    }

    gl.useProgram(program);

    // Quad geometry: two triangles covering the full viewport
    const vertices = new Float32Array([
      -1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1,
    ]);

    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

    const positionLoc = gl.getAttribLocation(program, "position");
    gl.enableVertexAttribArray(positionLoc);
    gl.vertexAttribPointer(positionLoc, 2, gl.FLOAT, false, 0, 0);

    // Uniform locations
    const resLoc = gl.getUniformLocation(program, "u_resolution");
    const timeLoc = gl.getUniformLocation(program, "u_time");
    const progLoc = gl.getUniformLocation(program, "u_progress");
    const procLoc = gl.getUniformLocation(program, "u_is_processing");

    let animationFrameId: number;
    const startTime = performance.now();

    // Responsive Canvas Resizing via ResizeObserver
    const resizeObserver = new ResizeObserver((entries) => {
      const entry = entries?.[0];
      if (!entry) return;
      const width = entry.contentRect.width;
      const height = entry.contentRect.height;

      const dpr = window.devicePixelRatio || 1;
      canvas.width = width * dpr;
      canvas.height = height * dpr;
      gl.viewport(0, 0, canvas.width, canvas.height);
    });

    resizeObserver.observe(container);

    let currentProgress = 1.0;
    let targetProgress = 1.0;

    // Render loop
    const render = (now: number) => {
      const elapsed = (now - startTime) / 1000;

      // Calculate target scan position
      if (parsedProgress >= 0) {
        // Map progress (0 to 1) directly, but invert it since webgl coordinates have y=0 at bottom and y=1 at top
        targetProgress = 1.0 - parsedProgress;
      } else {
        // Continuous scan loop from top to bottom (y=1 to y=0) using a smooth triangle wave
        const wave = Math.abs(((elapsed * 0.25) % 2.0) - 1.0); // goes 0 -> 1 -> 0
        targetProgress = wave;
      }

      // Smoothly interpolate currentProgress to targetProgress (Lerp / Easing)
      // This produces the exact same premium, fluid ease-out transition as GSAP!
      const lerpSpeed = isProcessing ? 0.08 : 0.04;
      currentProgress += (targetProgress - currentProgress) * lerpSpeed;

      gl.clearColor(0, 0, 0, 0);
      gl.clear(gl.COLOR_BUFFER_BIT);

      gl.useProgram(program);

      // Pass uniforms
      gl.uniform2f(resLoc, canvas.width, canvas.height);
      gl.uniform1f(timeLoc, elapsed);
      gl.uniform1f(progLoc, currentProgress);
      gl.uniform1i(procLoc, isProcessing ? 1 : 0);

      gl.drawArrays(gl.TRIANGLES, 0, 6);

      animationFrameId = requestAnimationFrame(render);
    };

    animationFrameId = requestAnimationFrame(render);

    // Clean up WebGL resources
    return () => {
      cancelAnimationFrame(animationFrameId);
      resizeObserver.disconnect();
      gl.deleteBuffer(buffer);
      gl.deleteProgram(program);
      gl.deleteShader(vs);
      gl.deleteShader(fs);
    };
  }, [parsedProgress, isProcessing, webglSupported]);

  return null;
}
