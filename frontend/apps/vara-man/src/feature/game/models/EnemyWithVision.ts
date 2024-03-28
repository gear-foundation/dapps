import { TileMap } from '../types'
import { Direction, Enemy, EnemyProps } from './Enemy'
import { Vec2 } from './Vec2'

export class EnemyWithVision extends Enemy {
	visionRadius: number
	playerPosition: Vec2

	constructor(props: EnemyProps, playerPosition: Vec2, visionRadius = 5) {
		super(props)
		this.playerPosition = playerPosition
		this.visionRadius = visionRadius
	}

	update(params: { mapData: TileMap; playerPosition: Vec2 }): void {
		this.playerPosition = params.playerPosition
		this.mapData = params.mapData

		if (this.isPlayerInVision()) {
			this.slideAlongWall(this.playerPosition)
			this.moveTowardsPlayer()

			this.legAnimation()
			this.armAnimation()
		} else {
			this.updateDirection()
			super.performMovement(this.isPlayerInVision())
		}
	}

	isPlayerInVision(): boolean {
		const directionToPlayer = Vec2.subtract(
			this.playerPosition,
			this.position
		).norm()
		const stepSize = 1
		const maxDistance = this.visionRadius * this.mapData.tilewidth

		let currentPoint = this.position.copy()
		let distanceTravelled = 0

		while (distanceTravelled < maxDistance) {
			currentPoint = currentPoint.add(directionToPlayer.scale(stepSize))
			distanceTravelled += stepSize

			const tileX = Math.floor(currentPoint.x / this.mapData.tilewidth)
			const tileY = Math.floor(currentPoint.y / this.mapData.tileheight)

			if (
				tileX < 0 ||
				tileX >= this.mapData.width ||
				tileY < 0 ||
				tileY >= this.mapData.height
			) {
				break
			}

			const tileIndex = tileY * this.mapData.width + tileX
			const tile = this.mapData.layers[0].data[tileIndex]
			if (tile === 1 || tile !== this.zone) {
				return false
			}

			if (Vec2.distance(currentPoint, this.playerPosition) <= stepSize) {
				return true
			}
		}

		return false
	}
	moveTowardsPlayer(): void {
		const directionX = this.playerPosition.x - this.position.x
		const directionY = this.playerPosition.y - this.position.y

		if (Math.abs(directionX) > Math.abs(directionY)) {
			this.direction = directionX > 0 ? Direction.right : Direction.left
		} else {
			this.direction = directionY > 0 ? Direction.down : Direction.up
		}

		const directionToPlayer = Vec2.subtract(this.playerPosition, this.position)
		const angleToPlayer = Math.atan2(directionToPlayer.y, directionToPlayer.x)

		this.rotation = angleToPlayer + Math.PI / 2

		this.velocity = this.calculateVelocityBasedOnDirection()
	}

	calculateVelocityBasedOnDirection(): Vec2 {
		switch (this.direction) {
			case Direction.up:
				return new Vec2(0, -this.speed)
			case Direction.down:
				return new Vec2(0, this.speed)
			case Direction.left:
				return new Vec2(this.speed, 0)
			case Direction.right:
				return new Vec2(-this.speed, 0)
			default:
				return new Vec2(0, 0)
		}
	}
}
