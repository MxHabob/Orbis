import { ProfileInfoState } from "./state/profile-info.svelte";
import { CalendarInfoState } from "./state/calendar-info.svelte";
import { FacilitiesInfoState } from "./state/facilities-info.svelte";
import { AcademicsInfoState } from "./state/academics-info.svelte";
import { FinanceInfoState } from "./state/finance-info.svelte";

class OnboardingState {
  profile = new ProfileInfoState();
  calendar = new CalendarInfoState();
  facilities = new FacilitiesInfoState();
  academics = new AcademicsInfoState();
  finance = new FinanceInfoState();

  reset() {
    this.profile = new ProfileInfoState();
    this.calendar = new CalendarInfoState();
    this.facilities = new FacilitiesInfoState();
    this.academics = new AcademicsInfoState();
    this.finance = new FinanceInfoState();
  }

  load() {
    // Load state if needed
  }
}

export const onboardingState = new OnboardingState();
