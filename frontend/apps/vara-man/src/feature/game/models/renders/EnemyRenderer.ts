import { Enemy } from '../Enemy';

export class EnemyRenderer {
  static render(context: CanvasRenderingContext2D, enemy: Enemy): void {
    const { position, rotation, torsoWidth, torsoHeight, legWidth, legs, armWidth, arms, headRadius, zone } = enemy;
    const colorHead = zone === 2 ? '#A40606' : '#A7A7A7';

    context.save();
    context.translate(position.x, position.y);
    context.rotate(rotation);

    // Legs
    legs.forEach((leg: { limb: string; height: number }) => {
      context.strokeStyle = '#1B4138';
      context.fillStyle = '#242424';
      context.beginPath();
      context.roundRect(
        leg.limb === 'left' ? -torsoWidth / 2 + legWidth / 2 : torsoWidth / 2 - legWidth - legWidth / 2,
        0,
        legWidth,
        leg.height / 2,
        5,
      );
      context.stroke();
      context.fill();
    });

    const radius = 5;

    // Torso
    context.beginPath();
    context.fillStyle = '#464646';
    context.moveTo(-torsoWidth / 2 + radius, -torsoHeight / 2);
    context.lineTo(torsoWidth / 2 - radius, -torsoHeight / 2);
    context.quadraticCurveTo(torsoWidth / 2, -torsoHeight / 2, torsoWidth / 2, -torsoHeight / 2 + radius);
    context.lineTo(torsoWidth / 2, torsoHeight / 2 - radius);
    context.quadraticCurveTo(torsoWidth / 2, torsoHeight / 2, torsoWidth / 2 - radius, torsoHeight / 2);
    context.lineTo(-torsoWidth / 2 + radius, torsoHeight / 2);
    context.quadraticCurveTo(-torsoWidth / 2, torsoHeight / 2, -torsoWidth / 2, torsoHeight / 2 - radius);
    context.lineTo(-torsoWidth / 2, -torsoHeight / 2 + radius);
    context.quadraticCurveTo(-torsoWidth / 2, -torsoHeight / 2, -torsoWidth / 2 + radius, -torsoHeight / 2);
    context.closePath();
    context.fill();

    // Hands
    arms.forEach((arm) => {
      context.strokeStyle = '#333333';
      context.fillStyle = '#333333';
      context.beginPath();
      context.roundRect(
        arm.limb === 'left' ? -torsoWidth / 2 : torsoWidth / 2 - armWidth,
        -torsoHeight / 4,
        armWidth,
        arm.height,
        5,
      );
      context.stroke();
      context.fill();
    });

    // Head
    context.beginPath();
    context.fillStyle = colorHead;
    context.arc(0, headRadius / 20, headRadius * 1.2, 0, Math.PI * 2, false);
    context.fill();

    // Hat or Head
    context.beginPath();
    context.fillStyle = '#272727';
    context.arc(0, 0, headRadius, 0, Math.PI * 2, false);
    context.fill();

    context.restore();

    // Drawing a border for debug
    // const bounds = enemy.getBounds()
    // context.strokeStyle = 'rgba(255, 0, 0, 1)'
    // context.strokeRect(bounds.x, bounds.y, bounds.width, bounds.height)
  }
}
