import { Character } from "../Character";

export class CharacterRenderer {
	static cloakImage: HTMLImageElement | null = null;

	static loadCloakImage(src: string): Promise<HTMLImageElement> {
		if (this.cloakImage) {
			return Promise.resolve(this.cloakImage);
		}

		return new Promise((resolve, reject) => {
			const img = new Image();
			img.onload = () => {
				this.cloakImage = img;
				resolve(img);
			};
			img.onerror = reject;
			img.src = src;
		});
	}

	static render(context: CanvasRenderingContext2D, character: Character): void {
		const { position, rotation } = character;

		context.save();
		context.translate(position.x, position.y);
		context.rotate(rotation);

		// Рендерим части тела
		this.renderLegs(context, character);
		this.renderArms(context, character);
		this.renderTorso(context, character);

		if (this.cloakImage) {
			this.renderCloak(context, character);
		}
		
		this.renderHead(context, character);

		

		context.restore();
	}

	static renderLegs(context: CanvasRenderingContext2D, character: Character): void {
		const { legs, torsoWidth, legWidth } = character;
		context.fillStyle = '#1B4138';
		context.strokeStyle = '#1B4138';

		legs.forEach((leg) => {
			context.beginPath();
			context.roundRect(
				leg.limb === 'left' ? -torsoWidth / 2 + legWidth / 2 : torsoWidth / 2 - legWidth - legWidth / 2,
				0,
				legWidth,
				leg.height / 2,
				5
			);
			context.stroke();
			context.fill();
		});
	}

	static renderArms(context: CanvasRenderingContext2D, character: Character): void {
		const { arms, torsoWidth, armWidth, torsoHeight } = character;
		context.fillStyle = '#00E3AE';
		context.strokeStyle = '#00E3AE';

		arms.forEach((arm) => {
			context.beginPath();
			context.roundRect(
				arm.limb === 'left' ? -torsoWidth / 2 : torsoWidth / 2 - armWidth,
				-torsoHeight / 4,
				armWidth,
				arm.height,
				5
			);
			context.stroke();
			context.fill();
		});
	}

	static renderTorso(context: CanvasRenderingContext2D, character: Character): void {
		const { torsoWidth, torsoHeight } = character;
		const radius = 5;
		context.fillStyle = '#00FFC4';

		context.beginPath();
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
	}

	static renderHead(context: CanvasRenderingContext2D, character: Character): void {
		const { headRadius } = character;
		context.fillStyle = '#000000';

		context.beginPath();
		context.arc(0, headRadius * 0.75 * -1, headRadius, 0, Math.PI * 2, false);
		context.fill();

		context.fillStyle = '#ffffff';
		context.beginPath();
		context.arc(0, 0, headRadius, 0, Math.PI * 2, false);
		context.fill();
	}

	static renderCloak(
		context: CanvasRenderingContext2D,
		character: Character
	): void {
		if (this.cloakImage && this.cloakImage.complete) {
			const { cloakAnimation } = character

			context.save()

			context.translate(0, 10)

			context.rotate(-Math.PI / 2)

			context.scale(cloakAnimation.scale, 1)
			context.drawImage(
				this.cloakImage,
				-this.cloakImage.naturalWidth / 2,
				-this.cloakImage.naturalHeight / 2
			)
			context.restore()
		}
	}

}
