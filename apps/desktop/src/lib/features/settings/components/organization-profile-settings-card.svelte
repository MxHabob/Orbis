<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";

interface Props {
  institutionType: string;
  educationalTrack: string;
  organizationSize: string;
  countryCode: string;
  countryName: string;
  ministryName: string;
  regionName: string;
  city: string;
  phone: string;
  website: string;
  taxId: string;
  licenseNumber: string;
  isSaving?: boolean;
}

let {
  institutionType = $bindable(),
  educationalTrack = $bindable(),
  organizationSize = $bindable(),
  countryCode = $bindable(),
  countryName = $bindable(),
  ministryName = $bindable(),
  regionName = $bindable(),
  city = $bindable(),
  phone = $bindable(),
  website = $bindable(),
  taxId = $bindable(),
  licenseNumber = $bindable(),
  isSaving = false,
}: Props = $props();
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.settings_general_section_profile()}</CardTitle>
  </CardHeader>
  <CardContent class="grid grid-cols-1 gap-5 p-6 text-start sm:grid-cols-2">
    <div class="flex flex-col gap-1.5">
      <Label for="institution-type" class="text-xs font-medium">
        {m.settings_general_institution_type()}
      </Label>
      <Select type="single" bind:value={institutionType} disabled={isSaving}>
        <SelectTrigger id="institution-type" class="w-full">
          {institutionType === "university"
            ? m.settings_general_institution_university()
            : institutionType === "institute"
              ? m.settings_general_institution_institute()
              : m.settings_general_institution_school()}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="school">{m.settings_general_institution_school()}</SelectItem>
          <SelectItem value="university">{m.settings_general_institution_university()}</SelectItem>
          <SelectItem value="institute">{m.settings_general_institution_institute()}</SelectItem>
        </SelectContent>
      </Select>
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="educational-track" class="text-xs font-medium">
        {m.settings_general_educational_track()}
      </Label>
      <Input id="educational-track" bind:value={educationalTrack} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="organization-size" class="text-xs font-medium">
        {m.settings_general_organization_size()}
      </Label>
      <Input id="organization-size" bind:value={organizationSize} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="country-code" class="text-xs font-medium">
        {m.settings_general_country_code()}
      </Label>
      <Input id="country-code" bind:value={countryCode} maxlength="2" disabled={isSaving} class="h-9 uppercase" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="country-name" class="text-xs font-medium">
        {m.settings_general_country_name()}
      </Label>
      <Input id="country-name" bind:value={countryName} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="ministry-name" class="text-xs font-medium">
        {m.settings_general_ministry_name()}
      </Label>
      <Input id="ministry-name" bind:value={ministryName} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="region-name" class="text-xs font-medium">
        {m.settings_general_region_name()}
      </Label>
      <Input id="region-name" bind:value={regionName} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="organization-city" class="text-xs font-medium">
        {m.settings_general_city()}
      </Label>
      <Input id="organization-city" bind:value={city} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="organization-phone" class="text-xs font-medium">
        {m.settings_general_phone()}
      </Label>
      <Input id="organization-phone" type="tel" bind:value={phone} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="organization-website" class="text-xs font-medium">
        {m.settings_general_website()}
      </Label>
      <Input id="organization-website" type="url" bind:value={website} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="organization-tax-id" class="text-xs font-medium">
        {m.settings_general_tax_id()}
      </Label>
      <Input id="organization-tax-id" bind:value={taxId} disabled={isSaving} class="h-9" />
    </div>

    <div class="flex flex-col gap-1.5">
      <Label for="organization-license-number" class="text-xs font-medium">
        {m.settings_general_license_number()}
      </Label>
      <Input id="organization-license-number" bind:value={licenseNumber} disabled={isSaving} class="h-9" />
    </div>
  </CardContent>
</Card>
