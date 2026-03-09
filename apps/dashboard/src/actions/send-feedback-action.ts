"use server";

import { actionClient } from "@/actions/safe-action";
import { z } from "zod";

const schema = z.object({
  feedback: z.string(),
});

export const sendFeebackAction = actionClient
  .schema(schema)
  .action(async ({ parsedInput: { feedback } }) => {
    // In a real application, save feedback to DB, send email, or external service.
    // Console log for demo purposes only:
    console.log("Feedback received:", feedback);
    
    // Simulating processing time
    await new Promise((resolve) => setTimeout(resolve, 500));
    
    return { success: true };
  });
