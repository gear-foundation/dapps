import { TileMap } from '../types'
import { Vec2 } from './Vec2'

interface LimbAnimation {
	limb: 'left' | 'right'
	height: number
	direction: 'up' | 'down'
}

const walkSpeed = 4

export class Character {
	position: Vec2
	velocity: Vec2
	rotation: number
	scale: number
	headRadius: number
	torsoWidth: number
	torsoHeight: number
	legWidth: number
	legHeight: number
	armWidth: number
	armHeight: number
	walkSpeed: number
	legs: LimbAnimation[]
	arms: LimbAnimation[]
	hero: boolean
	mapData: TileMap

	incrementCoins: (coin: 'silver' | 'gold') => void

	constructor(
		x: number,
		y: number,
		hero: boolean = false,
		mapData: TileMap,
		incrementCoins: (coin: 'silver' | 'gold') => void
	) {
		this.scale = 0.4
		this.position = new Vec2(x, y)
		this.velocity = new Vec2(0, walkSpeed * this.scale * -1)
		this.rotation = 0
		this.hero = hero
		this.mapData = mapData

		// Scaling dimensions
		this.headRadius = 20 * this.scale
		this.torsoWidth = 100 * this.scale
		this.torsoHeight = 30 * this.scale
		this.legWidth = 30 * this.scale
		this.legHeight = 80 * this.scale
		this.armWidth = 20 * this.scale
		this.armHeight = 30 * this.scale
		this.walkSpeed = walkSpeed * this.scale
		this.incrementCoins = incrementCoins

		// Initialize limb animations
		this.legs = [
			{
				limb: 'left',
				height: 0,
				direction: 'up',
			},
			{
				limb: 'right',
				height: 0,
				direction: 'down',
			},
		]
		this.arms = [
			{
				limb: 'left',
				height: 0,
				direction: 'down',
			},
			{
				limb: 'right',
				height: 0,
				direction: 'up',
			},
		]
	}

	legAnimation(isShift: boolean): void {
		this.legs.forEach((leg) => {
			const speedModifier = isShift ? this.walkSpeed * 2 : this.walkSpeed
			if (leg.height <= this.legHeight * -1) {
				leg.direction = 'down'
			} else if (leg.height >= this.legHeight) {
				leg.direction = 'up'
			}

			leg.height += leg.direction === 'down' ? speedModifier : -speedModifier
		})
	}

	armAnimation(isShift: boolean): void {
		this.arms.forEach((arm) => {
			const speedModifier = isShift
				? (this.walkSpeed / 2.8) * 2
				: this.walkSpeed / 2.8
			if (arm.height <= this.armHeight * -1) {
				arm.direction = 'down'
			} else if (arm.height >= this.armHeight) {
				arm.direction = 'up'
			}

			arm.height += arm.direction === 'down' ? speedModifier : -speedModifier
		})
	}

	updateMovement(
		isLeft: boolean,
		isRight: boolean,
		isUp: boolean,
		isDown: boolean,
		isShift: boolean
	): void {
		if (isLeft) {
			this.rotation -= Math.PI * 0.02
		}

		if (isRight) {
			this.rotation += Math.PI * 0.02
		}

		if (isUp || isDown) {
			const direction = isUp ? -1 : 1
			const nextVelocity = new Vec2(
				Math.cos(this.rotation + Math.PI / 2),
				Math.sin(this.rotation + Math.PI / 2)
			).mult(this.walkSpeed * direction * (isShift ? 2 : 1))
			const nextPosition = Vec2.add(this.position, nextVelocity)

			const isCollision = this.checkCollision(nextPosition)

			this.legAnimation(isShift)
			this.armAnimation(isShift)

			if (!isCollision) {
				this.velocity = nextVelocity
				this.position.add(this.velocity)

				this.checkForCoinCollection()
			} else {
				let horizontalMovement = new Vec2(nextVelocity.x, 0)
				let verticalMovement = new Vec2(0, nextVelocity.y)

				let horizontalCollision = this.checkCollision(
					Vec2.add(this.position, horizontalMovement)
				)
				let verticalCollision = this.checkCollision(
					Vec2.add(this.position, verticalMovement)
				)

				if (!horizontalCollision && verticalCollision) {
					this.position.add(horizontalMovement)
				} else if (horizontalCollision && !verticalCollision) {
					this.position.add(verticalMovement)
				} else {
					this.velocity = new Vec2(0, 0)
				}
			}
		}
	}

	public checkCollision(nextPosition: Vec2): boolean {
		const left = Math.floor(
			(nextPosition.x - this.torsoWidth / 3) / this.mapData.tilewidth
		)
		const right = Math.floor(
			(nextPosition.x + this.torsoWidth / 3) / this.mapData.tilewidth
		)
		const top = Math.floor(
			(nextPosition.y - this.torsoHeight / 1) / this.mapData.tileheight
		)
		const bottom = Math.floor(
			(nextPosition.y + this.torsoHeight / 1) / this.mapData.tileheight
		)

		for (let y = top; y <= bottom; y++) {
			for (let x = left; x <= right; x++) {
				const tileIndex = y * this.mapData.width + x
				const tileValue = this.mapData.layers[0].data[tileIndex]
				if (tileValue === 1) {
					return true
				}
			}
		}

		if (
			nextPosition.x < 0 ||
			nextPosition.y < 0 ||
			nextPosition.x > this.mapData.width * this.mapData.tilewidth ||
			nextPosition.y > this.mapData.height * this.mapData.tileheight
		) {
			return true
		}

		return false
	}

	checkForCoinCollection() {
		const tileSize = this.mapData.tilewidth
		const mapWidth = Math.sqrt(this.mapData.layers[1].data.length)

		const mapX = Math.floor(this.position.x / tileSize)
		const mapY = Math.floor(this.position.y / tileSize)

		const coinIndex = mapY * mapWidth + mapX

		const goldCoins = [
			11, 12, 13, 14, 25, 26, 27, 28, 39, 40, 41, 42, 53, 54, 55, 56,
		]
		const silverCoins = [22, 23, 36, 37]

		const allCoins = [...goldCoins, ...silverCoins]

		const tileValue = this.mapData.layers[1].data[coinIndex]

		if (goldCoins.includes(tileValue)) {
			this.incrementCoins('gold')
			this.removeCoinTiles(coinIndex, allCoins)
		} else if (silverCoins.includes(tileValue)) {
			this.incrementCoins('silver')
			this.removeCoinTiles(coinIndex, allCoins)
		}
	}

	removeCoinTiles(coinIndex: number, allCoins: number[]) {
		const mapWidth = this.mapData.width
		const mapHeight = this.mapData.height
		const coinTiles = new Set(allCoins)

		const visited = new Set()
		const queue = [coinIndex]

		while (queue.length > 0) {
			const index = queue.shift()

			if (index) {
				if (visited.has(index)) continue
				visited.add(index)

				const tileId = this.mapData.layers[1].data[index]
				if (!coinTiles.has(tileId)) continue

				this.mapData.layers[1].data[index] = 0

				const x = index % mapWidth
				const y = Math.floor(index / mapWidth)

				;[
					[-1, 0],
					[1, 0],
					[0, -1],
					[0, 1],
				].forEach(([dx, dy]) => {
					const nx = x + dx,
						ny = y + dy
					if (nx >= 0 && nx < mapWidth && ny >= 0 && ny < mapHeight) {
						const nIndex = ny * mapWidth + nx
						if (
							!visited.has(nIndex) &&
							coinTiles.has(this.mapData.layers[1].data[nIndex])
						) {
							queue.push(nIndex)
						}
					}
				})
			}
		}
	}
}
