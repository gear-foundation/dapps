import { Character } from './Character'

import { CharacterRenderer } from './renders/CharacterRenderer'
import { MapRenderer } from './renders/MapRenderer'
import { EnemyRenderer } from './renders/EnemyRenderer'
import { EnemyWithVision } from './EnemyWithVision'

import { findEnemyStartPositions } from '../utils/findEnemyStartPositions'
import { findCharacterStartPosition } from '../utils/findCharacterStartPosition'

import { IGameLevel } from '@/app/types/game'
import { TileMap } from '../types'
import { gameLevels } from '../consts'

const WIDTH_CANVAS = 588
const HEIGHT_CANVAS = 588

export class Game {
	private context: CanvasRenderingContext2D
	private fogContext: CanvasRenderingContext2D

	private character: Character | undefined
	private enemies: EnemyWithVision[] = []
	private animationFrameId: number | null = null

	private isUp = false
	private isDown = false
	private isLeft = false
	private isRight = false
	private isShift = false

	map: TileMap
	level: IGameLevel

	setGameOver = (gameOver: boolean) => {}
	gameOver = false
	pause?: boolean

	constructor(
		private canvas: HTMLCanvasElement,
		private canvasFog: HTMLCanvasElement,

		level: IGameLevel,
		incrementCoins: (coin: 'silver' | 'gold') => void,
		gameOver: boolean,
		setGameOver: (gameOver: boolean) => void,
		map: TileMap,
		pause?: boolean
	) {
		const levelData = gameLevels.find((l) => {
			return l.level === level
		})

		this.map = map
		this.level = level

		this.context = canvas.getContext('2d') as CanvasRenderingContext2D
		this.fogContext = canvasFog.getContext('2d') as CanvasRenderingContext2D
		this.canvas.width = WIDTH_CANVAS
		this.canvas.height = HEIGHT_CANVAS

		this.canvasFog.width = WIDTH_CANVAS
		this.canvasFog.height = HEIGHT_CANVAS

		this.setGameOver = setGameOver
		this.gameOver = gameOver
		this.pause = pause

		// Get the DPR and size of the canvas
		const dpr = window.devicePixelRatio
		const rect = canvas.getBoundingClientRect()

		// Set the "actual" size of the canvas
		canvas.width = rect.width * dpr
		canvas.height = rect.height * dpr

		// Scale the context to ensure correct drawing operations
		this.context.scale(dpr, dpr)

		// Set the "drawn" size of the canvas
		canvas.style.width = `${rect.width}px`
		canvas.style.height = `${rect.height}px`

		MapRenderer.initTilesets(this.map).then(() => {
			const startPosition = findCharacterStartPosition(this.map)
			const enemyStartPositions = findEnemyStartPositions(this.map)

			if (startPosition) {
				this.character = new Character(
					startPosition.x,
					startPosition.y,
					true,
					this.map,
					incrementCoins,
					() => this.setGameOver(true)
				)

				this.initEventListeners()
			} else {
				console.error('The character starting position was not found.')
			}

			enemyStartPositions.forEach(({ position, zone }) => {
				if (this.character) {
					const enemy = new EnemyWithVision(
						{
							x: position.x,
							y: position.y,
							zone: zone,
							speed: levelData!.speed,
							mapData: this.map,
						},
						this.character.position,
						levelData!.visionEnemy
					)
					this.enemies.push(enemy)
				}
			})

			CharacterRenderer.loadCloakImage('./cloak.svg')
				.then((img) => {
					CharacterRenderer.cloakImage = img
					this.update()
				})
				.catch((error) => {
					console.error(error)
					this.update()
				})
		})
	}

	private initEventListeners() {
		window.addEventListener('keydown', this.handleKeyDown)
		window.addEventListener('keyup', this.handleKeyUp)
	}

	private handleKeyDown = (event: KeyboardEvent) => {
		event.preventDefault()
		switch (event.keyCode) {
			case 38:
				this.isUp = true
				break
			case 40:
				this.isDown = true
				break
			case 37:
				this.isLeft = true
				break
			case 39:
				this.isRight = true
				break
			case 16:
				this.isShift = true
				break
		}
	}

	private handleKeyUp = (event: KeyboardEvent) => {
		event.preventDefault()
		switch (event.keyCode) {
			case 38:
				this.isUp = false
				break
			case 40:
				this.isDown = false
				break
			case 37:
				this.isLeft = false
				break
			case 39:
				this.isRight = false
				break
			case 16:
				this.isShift = false
				break
		}
	}

	private update = () => {
		if (this.gameOver) {
			this.cleanup()
			return
		}

		if (this.animationFrameId !== null) {
			if (!this.pause) {
				if (this.character) {
					this.character.updateMovement(
						this.isLeft,
						this.isRight,
						this.isUp,
						this.isDown,
						this.isShift
					)
				}

				this.enemies.forEach((enemy) => {
					if (this.character) {
						enemy.update({
							mapData: this.map,
							playerPosition: this.character.position,
						})
					}
				})

				if (this.checkCollisions()) {
					this.setGameOver(true)
					return
				}
				this.context.clearRect(0, 0, this.canvas.width, this.canvas.height)
			}
		}

		this.animationFrameId = requestAnimationFrame(this.update)
		this.render()
	}

	private render() {
		if (this.character) {
			MapRenderer.render(this.context, this.map)
			CharacterRenderer.render(this.context, this.character)

			this.enemies.forEach((enemy) => EnemyRenderer.render(this.context, enemy))

			if (this.level === 'Hard') {
				MapRenderer.renderFogOfWar(
					this.fogContext,
					this.character.position,
					150
				)
			}
		}
	}

	public cleanup() {
		if (this.animationFrameId !== null) {
			cancelAnimationFrame(this.animationFrameId)
			this.animationFrameId = null
		}

		window.removeEventListener('keydown', this.handleKeyDown)
		window.removeEventListener('keyup', this.handleKeyUp)
	}

	checkCollisions() {
		if (!this.character) return false

		const characterBounds = this.character.getBounds()

		for (const enemy of this.enemies) {
			const enemyBounds = enemy.getBounds()

			if (
				characterBounds.x < enemyBounds.x + enemyBounds.width / 4 &&
				characterBounds.x + characterBounds.width > enemyBounds.x &&
				characterBounds.y < enemyBounds.y + enemyBounds.height / 4 &&
				characterBounds.y + characterBounds.height > enemyBounds.y
			) {
				return true
			}
		}

		return false
	}

	public updateGameOver = (gameOver: boolean) => {
		this.gameOver = gameOver
		if (gameOver) {
			this.cleanup()
		}
	}

	public updatePause = () => {
		this.pause = false
	}
}
