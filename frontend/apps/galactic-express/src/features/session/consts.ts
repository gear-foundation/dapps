import { ReactComponent as AltitudeSVG } from './assets/altitude.svg';
import { ReactComponent as WeatherSVG } from './assets/weather.svg';
import { ReactComponent as RewardSVG } from './assets/reward.svg';
import { isGreaterThanZero } from './utils';

const TRAITS = [
  { heading: 'Altitude', SVG: AltitudeSVG },
  { heading: 'Weather', SVG: WeatherSVG },
  { heading: 'Reward', SVG: RewardSVG },
];

// const WEATHERS = ['Sunny ☀️', 'Cloudy ☁️', 'Rainy 🌦️', 'Storm 🌧️', 'Thunder ⛈️', 'Tornado 🌪️'];

const WEATHERS = {
  Clear: {
    weight: 0,
    name: 'Sunny ☀️',
  },
  Cloudy: {
    weight: 1,
    name: 'Cloudy ☁️',
  },
  Rainy: {
    weight: 2,
    name: 'Rainy 🌦️',
  },
  Stormy: {
    weight: 3,
    name: 'Storm 🌧️',
  },
  Thunder: {
    weight: 4,
    name: 'Thunder ⛈️',
  },
  Tornado: {
    weight: 5,
    name: 'Tornado 🌪️',
  },
};

const INITIAL_VALUES = {
  payload: '0',
  fuel: '0',
};

const VALIDATE = {
  payload: isGreaterThanZero,
  fuel: isGreaterThanZero,
};

const TABLE_HEADINGS = ['Player', 'Name', 'Alive', 'Fuel Left', 'Altitude', 'Payload'];

const PLAYER_COLORS = ['#eb5757', '#f2c94c', '#2f80ed', '#9b51e0'];

export { TRAITS, WEATHERS, INITIAL_VALUES, VALIDATE, TABLE_HEADINGS, PLAYER_COLORS };
