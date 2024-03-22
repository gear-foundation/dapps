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
		this.canvas.width = 588
		this.canvas.height = 588

		this.canvasFog.width = 588
		this.canvasFog.height = 588

		this.setGameOver = setGameOver
		this.gameOver = gameOver
		this.pause = pause

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

			CharacterRenderer.loadCloakImage('./public/cloak.svg')
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
