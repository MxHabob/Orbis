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
    this.profile.reset();
    this.calendar.reset();
    this.facilities.reset();
    this.academics.reset();
    this.finance.reset();
  }
}

export const onboardingState = new OnboardingState();
