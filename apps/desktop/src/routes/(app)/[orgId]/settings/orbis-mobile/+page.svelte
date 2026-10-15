<script lang="ts">
  import * as m from "$lib/paraglide/messages";
</script>

<svelte:head>
  <title>{m.settings_orbis_mobile_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background">
  <div class="mx-auto flex w-full max-w-4xl flex-col gap-6 p-6">
    <header class="space-y-2">
      <h1 class="text-xl font-semibold text-foreground">{m.settings_orbis_mobile_title()}</h1>
      <p class="max-w-3xl text-sm leading-6 text-muted-foreground">
        {m.settings_orbis_mobile_description()}
      </p>
    </header>

    <section class="space-y-4 rounded-xl border border-border bg-card p-5">
      <div>
        <h2 class="text-sm font-semibold text-foreground">
          {m.settings_orbis_mobile_directory_title()}
        </h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
          {m.settings_orbis_mobile_directory_description()}
        </p>
      </div>
      <ul class="list-disc space-y-2 ps-5 text-sm leading-6 text-muted-foreground">
        <li>{m.settings_orbis_mobile_directory_step_one()}</li>
        <li>{m.settings_orbis_mobile_directory_step_two()}</li>
        <li>{m.settings_orbis_mobile_directory_step_three()}</li>
      </ul>
    </section>

    <p class="rounded-lg border border-primary/20 bg-primary/5 p-4 text-sm leading-6 text-muted-foreground">
      {m.settings_orbis_mobile_privacy()}
    </p>
  </div>
</div>
