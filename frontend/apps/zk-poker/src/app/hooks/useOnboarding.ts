import { atom, useAtom } from 'jotai';

const ONBOARDING_PASSED_KEY = 'isOnboardingPassed';

const onboardingAtom = atom(localStorage.getItem(ONBOARDING_PASSED_KEY) === 'true');

const useOnboarding = () => {
  const [isOnboardingPassed, setIsOnboardingPassed] = useAtom(onboardingAtom);

  return {
    isOnboardingPassed,
    setOnboardingPassed: () => {
      localStorage.setItem(ONBOARDING_PASSED_KEY, 'true');
      setIsOnboardingPassed(true);
    },
  };
};

export { useOnboarding };
