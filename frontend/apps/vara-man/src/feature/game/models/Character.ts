import { TileMap } from '../types'
import { Vec2 } from './Vec2'

interface LimbAnimation {
	limb: 'left' | 'right'
	height: number
	direction: 'up' | 'down'
}

interface CloakAnimation {
	scale: number
	direction: 'increasing' | 'decreasing'
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
	clock: LimbAnimation[]
	hero: boolean
	mapData: TileMap

	incrementCoins: (coin: 'silver' | 'gold') => void
	setGameOver: (gameOver: boolean) => void
	totalCoins: number
	collectedCoins: number = 0

	cloakAnimation: CloakAnimation = {
		scale: 1.0,
		direction: 'decreasing',
	}

	constructor(
		x: number,
		y: number,
		hero: boolean = false,
		mapData: TileMap,
		incrementCoins: (coin: 'silver' | 'gold') => void,
		setGameOver: (gameOver: boolean) => void
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
		this.totalCoins = this.countTotalCoins()
		this.collectedCoins = 0
		this.setGameOver = setGameOver

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
		this.clock = [
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

	getBounds() {
		const padding = 4
		return {
			x: this.position.x - this.torsoWidth / 2 + padding,
			y: this.position.y - this.torsoHeight * 2 + padding,
			width: this.torsoWidth - padding * 2,
			height: this.torsoHeight + this.legHeight - padding * 2,
		}
	}

	cloakAnimationStep(): void {
		const speedModifier = 0.02

		if (this.cloakAnimation.scale <= 0.4) {
			this.cloakAnimation.direction = 'increasing'
		} else if (this.cloakAnimation.scale >= 1.3) {
			this.cloakAnimation.direction = 'decreasing'
		}

		if (this.cloakAnimation.direction === 'decreasing') {
			this.cloakAnimation.scale -= speedModifier
		} else {
			this.cloakAnimation.scale += speedModifier
		}
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
			this.rotation -= Math.PI * 0.015
		}

		if (isRight) {
			this.rotation += Math.PI * 0.015
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
			this.cloakAnimationStep()

			this.checkForCoinCollection()
			if (!isCollision) {
				this.velocity = nextVelocity
				this.position.add(this.velocity)
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
		const bounds = this.getBounds()
		bounds.x += nextPosition.x - this.position.x
		bounds.y += nextPosition.y - this.position.y

		const left = Math.floor(bounds.x / this.mapData.tilewidth)
		const right =
			Math.ceil((bounds.x + bounds.width) / this.mapData.tilewidth) - 1
		const top = Math.floor(bounds.y / this.mapData.tileheight)
		const bottom =
			Math.ceil((bounds.y + bounds.height) / this.mapData.tileheight) - 1

		for (let y = top; y <= bottom; y++) {
			for (let x = left; x <= right; x++) {
				if (
					x >= 0 &&
					x < this.mapData.width &&
					y >= 0 &&
					y < this.mapData.height
				) {
					const tileIndex = y * this.mapData.width + x
					const tileValue = this.mapData.layers[0].data[tileIndex]
					if (tileValue === 1) {
						return true
					}
				}
			}
		}

		if (
			bounds.x < 0 ||
			bounds.y < 0 ||
			bounds.x + bounds.width > this.mapData.width * this.mapData.tilewidth ||
			bounds.y + bounds.height > this.mapData.height * this.mapData.tileheight
		) {
			return true
		}

		return false
	}

	checkForCoinCollection() {
		const bounds = this.getBounds()
		const leftTile = Math.floor(bounds.x / this.mapData.tilewidth)
		const rightTile =
			Math.ceil((bounds.x + bounds.width) / this.mapData.tilewidth) - 1
		const topTile = Math.floor(bounds.y / this.mapData.tileheight)
		const bottomTile =
			Math.ceil((bounds.y + bounds.height) / this.mapData.tileheight) - 1

		const goldCoins = [
			11, 12, 13, 14, 25, 26, 27, 28, 39, 40, 41, 42, 53, 54, 55, 56,
		]
		const silverCoins = [
			7, 8, 9, 10, 21, 22, 23, 24, 35, 36, 37, 38, 49, 50, 51, 52,
		]
		for (let y = topTile; y <= bottomTile; y++) {
			for (let x = leftTile; x <= rightTile; x++) {
				const tileIndex = y * this.mapData.width + x
				const tileValue = this.mapData.layers[1].data[tileIndex]

				if (goldCoins.includes(tileValue)) {
					this.incrementCoins('gold')
					this.collectedCoins += 1
					this.removeCoinTiles(tileIndex, [...goldCoins, ...silverCoins])
					return
				} else if (silverCoins.includes(tileValue)) {
					this.incrementCoins('silver')
					this.collectedCoins += 1
					this.removeCoinTiles(tileIndex, [...goldCoins, ...silverCoins])
					return
				}
			}
		}
	}

	// TODO: Change the method of removing the entire coin
	removeCoinTiles(coinIndex: number, allCoins: number[]) {
		console.log('this.totalCoins', this.collectedCoins)
		if (this.collectedCoins === this.totalCoins) {
			console.log('collectedCoins', this.collectedCoins)
			this.setGameOver(true)
		}

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

				for (let i = 0; i < 4; i++) {
					for (let j = 0; j < 4; j++) {
						const x = (index % mapWidth) + i
						const y = Math.floor(index / mapWidth) + j
						if (x >= 0 && x < mapWidth && y >= 0 && y < mapHeight) {
							const nIndex = y * mapWidth + x
							this.mapData.layers[1].data[nIndex] = 0
						}
					}
				}

				;[
					[-1, 0],
					[1, 0],
					[0, -1],
					[0, 1],
				].forEach(([dx, dy]) => {
					const nx = (index % mapWidth) + dx,
						ny = Math.floor(index / mapWidth) + dy
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

	countTotalCoins(): number {
		const oneCoinTiles = 16

		let count = 0
		for (let layer of this.mapData.layers) {
			if (layer.name === 'coins') {
				for (let tile of layer.data) {
					if (tile > 0) count++
				}
			}
		}

		// TODO: The map has a problem, if the first array starts with a coin, the character won't be able to get one
		return 84
	}
}
