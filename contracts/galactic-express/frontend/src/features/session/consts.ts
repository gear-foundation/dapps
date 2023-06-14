import { ReactComponent as AltitudeSVG } from './assets/altitude.svg';
import { ReactComponent as WeatherSVG } from './assets/weather.svg';
import { ReactComponent as FuelSVG } from './assets/fuel.svg';
import { ReactComponent as RewardSVG } from './assets/reward.svg';
import { isGreaterThanZero } from './utils';

const TRAITS = [
  { heading: 'Altitude', SVG: AltitudeSVG },
  { heading: 'Weather', SVG: WeatherSVG },
  { heading: 'Fuel', SVG: FuelSVG },
  { heading: 'Reward', SVG: RewardSVG },
];

const WEATHERS = ['Sunny ☀️', 'Cloudy ☁️', 'Rainy 🌦️', 'Storm 🌧️', 'Thunder ⛈️', 'Tornado 🌪️'];

const INITIAL_VALUES = {
  payload: '0',
  fuel: '0',
};

const VALIDATE = {
  deposit: isGreaterThanZero,
  payload: isGreaterThanZero,
  fuel: isGreaterThanZero,
};

export { TRAITS, WEATHERS, INITIAL_VALUES, VALIDATE };
