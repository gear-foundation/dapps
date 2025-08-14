import clsx from 'clsx';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { useOnboarding } from '@/app/hooks';
import {
  OnboardingLockIcon,
  VaraLogoIcon,
  SpadeLgIcon,
  OnboardingCardsIcon,
  OnboardingBlackjackIcon,
} from '@/assets/images';
import { Button } from '@/components';

import styles from './onboarding.module.scss';

const config = [
  {
    title: 'Secure Blockchain Poker',
    description: 'Experience truly fair gameplay with zero-knowledge proofs technology',
    image: OnboardingLockIcon,
  },
  {
    title: 'Guaranteed Fair Play',
    description: 'Every card dealt is cryptographically secured and provably random.',
    image: OnboardingCardsIcon,
  },
  {
    title: 'Join the Community',
    description: 'Create your own tables or play with friends using test PTS tokens.',
    image: OnboardingBlackjackIcon,
  },
];

const OnboardingPage = () => {
  const [currentStep, setCurrentStep] = useState(0);
  const navigate = useNavigate();
  const { setOnboardingPassed } = useOnboarding();

  const handleSkip = () => {
    setOnboardingPassed();
    navigate(ROUTES.LOGIN);
  };

  const handleContinue = () => {
    if (currentStep < config.length - 1) {
      setCurrentStep(currentStep + 1);
    } else {
      setOnboardingPassed();
      navigate(ROUTES.LOGIN);
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.glow} />
      <div className={clsx(styles.glow, styles.bottom)} />
      <div className={styles.shadow} />

      <div className={styles.wrapper}>
        <div className={styles.logo}>
          <VaraLogoIcon />
        </div>

        <div className={styles.imageContainer}>
          <img src={config[currentStep].image} alt={config[currentStep].title} />
        </div>

        <div className={styles.content}>
          <div className={clsx(styles.pagination, styles[`step${currentStep}`])}>
            {config.map((_, index) => (
              <div className={styles.paginationItem} key={index}>
                <div className={clsx(styles.dot, currentStep === index && styles.active)} />
              </div>
            ))}
            <div className={styles.paginationTrack}>
              <SpadeLgIcon className={styles.spadeIcon} />
            </div>
          </div>

          <h1 className={styles.title}>{config[currentStep].title}</h1>
          <p className={styles.description}>{config[currentStep].description}</p>

          <div className={styles.actions}>
            <Button size="small" color="transparent" onClick={handleSkip}>
              Skip
            </Button>
            <Button onClick={handleContinue}>Continue</Button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default OnboardingPage;
