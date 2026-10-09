# AI Tool Authorization Contract

Every registered AI tool must declare a permission string that resolves to a known IAM `Permission`. Missing or unknown permissions deny discovery and execution.

`GET /api/v1/ai/tools` returns only tools permitted to the authenticated actor. `POST /api/v1/ai/chat` supplies that same authorized set to the model and checks the selected tool again before staging a consequential action or dispatching a read/generation tool. `execute_tool` repeats the check at its server-side boundary; model output and user confirmation do not grant permission.

Data returned by tools must be queried through organization-scoped services. Direct lookups must verify the returned record's organization before including it in a tool response or prompt. Role assignments, role permissions, group memberships, and group permissions are likewise resolved within the request organization.

Teacher access to students and class-based analysis is limited to active enrollment classes linked to the teacher's employee record and verified class-subject assignments (including homeroom assignments). A client-supplied class, subject, or class-subject id is only a selector; the server verifies tenant ownership and the teacher's assignment before using it.

Users whose role assignments are scoped to organizational units receive only tools with explicit unit-scoped execution paths. Student and class records must be in the assigned unit or its descendants. Organization-wide tools without a safe unit filter are omitted from discovery and rejected at execution. Invalid or cross-organization unit ids do not widen scope.

An authorization denial omits a tool from discovery or rejects its execution. An IAM/backend error is returned rather than treated as an authorization success.

## Financial query and report handling

`get_unpaid_fees` requires `finance:read` and is unavailable to users with organizational-unit restrictions until the fee query can be safely scoped to those units. It returns no more than 50 detail rows to the UI. The details are carried separately from tool output and are not added to model history or the locally saved chat transcript. More than 50 matches require `data:export` and create an XLSX report job instead.

Report job creation requires `data:export`; financial report definitions also require `finance:read`. Status and download requests re-check those permissions and require the same organization and creator as the job. Status responses never include the server file path. Report files are held under an organization-specific temporary directory and downloads expire 24 hours after job creation; an expired file is removed when an expired download is requested.

## Notification integration selection

Notification callers may provide an explicit integration ID. The unified adapter resolves it only among the request organization's connected integrations, rejects disabled integrations and providers without a registered adapter for the selected channel, and never substitutes another integration when an explicit ID is invalid. Provider credentials are validated by the selected provider before it sends.

`GET /api/v1/integrations/notifications?channel=<channel>` requires `notification:write` and reports connected integrations with their supported/eligible status. The health check performs a credential-authenticated provider request with a 10-second timeout; its response exposes only a coarse failure reason, never provider response bodies or credentials. Currently registered delivery channels are SendGrid `email` and Twilio `sms`/`whatsapp`. Other channels are reported as unsupported until a provider adapter is registered.
