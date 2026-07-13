<script lang="ts">
import * as m from "$lib/paraglide/messages";

</script>

<!-- Top Announcement Banner -->
<div class="w-full bg-primary text-primary-foreground py-2 px-4 text-center text-[11px] font-semibold flex items-center justify-center gap-2 transition-all duration-200 shrink-0 select-none z-10">
	<span>{m.auth_login_banner_text()}</span>
	<a
		href="https://orbis.ai/blog"
		target="_blank"
		rel="noreferrer"
		class="underline hover:opacity-90 transition-opacity flex items-center gap-0.5"
	>
		{m.auth_login_banner_link()}
		<span class="rtl:rotate-180 inline-block font-sans">→</span>
	</a>
</div>
