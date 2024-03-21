import { Character } from '../Character'

export class CharacterRenderer {
	static render(context: CanvasRenderingContext2D, character: Character): void {
		const {
			position,
			rotation,
			torsoWidth,
			torsoHeight,
			legWidth,
			legs,
			armWidth,
			arms,
			headRadius,
		} = character

		context.save()
		context.translate(position.x, position.y)
		context.rotate(rotation)

		// Legs
		legs.forEach((leg: { limb: string; height: number }) => {
			context.strokeStyle = '#1B4138'
			context.fillStyle = '#1B4138'
			context.beginPath()
			context.roundRect(
				leg.limb === 'left'
					? -torsoWidth / 2 + legWidth / 2
					: torsoWidth / 2 - legWidth - legWidth / 2,
				0,
				legWidth,
				leg.height / 2,
				5
			)
			context.stroke()
			context.fill()
		})

		const radius = 5

		// Hands
		arms.forEach((arm: { limb: string; height: number }) => {
			context.strokeStyle = '#00E3AE'
			context.fillStyle = '#00E3AE'
			context.beginPath()
			context.roundRect(
				arm.limb === 'left' ? -torsoWidth / 2 : torsoWidth / 2 - armWidth,
				-torsoHeight / 4,
				armWidth,
				arm.height,
				5
			)
			context.stroke()
			context.fill()
		})

		// Torso
		context.beginPath()
		context.fillStyle = '#00FFC4'
		context.moveTo(-torsoWidth / 2 + radius, -torsoHeight / 2)
		context.lineTo(torsoWidth / 2 - radius, -torsoHeight / 2)
		context.quadraticCurveTo(
			torsoWidth / 2,
			-torsoHeight / 2,
			torsoWidth / 2,
			-torsoHeight / 2 + radius
		)
		context.lineTo(torsoWidth / 2, torsoHeight / 2 - radius)
		context.quadraticCurveTo(
			torsoWidth / 2,
			torsoHeight / 2,
			torsoWidth / 2 - radius,
			torsoHeight / 2
		)
		context.lineTo(-torsoWidth / 2 + radius, torsoHeight / 2)
		context.quadraticCurveTo(
			-torsoWidth / 2,
			torsoHeight / 2,
			-torsoWidth / 2,
			torsoHeight / 2 - radius
		)
		context.lineTo(-torsoWidth / 2, -torsoHeight / 2 + radius)
		context.quadraticCurveTo(
			-torsoWidth / 2,
			-torsoHeight / 2,
			-torsoWidth / 2 + radius,
			-torsoHeight / 2
		)
		context.closePath()
		context.fill()

		// Head
		context.beginPath()
		context.fillStyle = '#000000'
		context.arc(0, headRadius * 0.75 * -1, headRadius, 0, Math.PI * 2, false)
		context.fill()

		// Hat or Head
		context.beginPath()
		context.fillStyle = '#ffffff'
		context.arc(0, 0, headRadius, 0, Math.PI * 2, false)
		context.fill()

		context.restore()

		// Drawing a border for debug
		const bounds = character.getBounds()
		context.strokeStyle = 'rgba(255, 0, 0, 0.5)'
		context.strokeRect(bounds.x, bounds.y, bounds.width, bounds.height)
	}
}
