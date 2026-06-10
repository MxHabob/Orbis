import { createAuthClient } from "better-auth/react"
import { magicLinkClient } from "better-auth/client/plugins";

export const authClient = createAuthClient({
    baseURL: process.env.NEXT_PUBLIC_API_URL || "https://orbis-api.mulverse.com",
    plugins: [
        magicLinkClient()
    ]
})