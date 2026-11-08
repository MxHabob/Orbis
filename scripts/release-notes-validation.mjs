export function validateReleaseNotes(markdown, description) {
  const withoutComments = markdown.replace(/<!--[\s\S]*?-->/g, "").trim();
  if (/TODO|TBD/i.test(withoutComments)) {
    throw new Error(`${description} still contains TODO/TBD text.`);
  }

  const sections = new Map();
  let currentHeading = null;
  for (const line of withoutComments.split(/\r?\n/)) {
    const heading = /^##\s+(.+?)\s*$/.exec(line);
    if (heading) {
      currentHeading = heading[1].toLowerCase();
      sections.set(currentHeading, []);
    } else if (currentHeading) {
      sections.get(currentHeading).push(line);
    }
  }

  for (const required of ["summary", "known issues"]) {
    const content = sections.get(required)?.join("\n").trim();
    if (!content)
      throw new Error(
        `${description} must include non-empty "## ${required === "summary" ? "Summary" : "Known Issues"}" content.`,
      );
  }
}
