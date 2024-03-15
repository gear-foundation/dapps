import { TileMap } from '../../types'

class Tileset {
	image: HTMLImageElement
	tileWidth: number
	tileHeight: number
	imageWidth: number
	imageHeight: number
	firstgid: number
	tilecount: number

	constructor(
		src: string,
		tileWidth: number,
		tileHeight: number,
		imageWidth: number,
		imageHeight: number,
		firstgid: number,
		tilecount: number
	) {
		this.image = new Image()
		this.image.src = src
		this.tileWidth = tileWidth
		this.tileHeight = tileHeight
		this.imageWidth = imageWidth
		this.imageHeight = imageHeight
		this.firstgid = firstgid
		this.tilecount = tilecount
	}
}

export class MapRenderer {
	private static tilesets: Tileset[] = []

	public static async initTilesets(mapData: TileMap) {
		this.tilesets = mapData.tilesets.map(
			(tileset) =>
				new Tileset(
					tileset.image,
					tileset.tilewidth,
					tileset.tileheight,
					tileset.imagewidth,
					tileset.imageheight,
					tileset.firstgid,
					tileset.tilecount
				)
		)

		await Promise.all(
			this.tilesets.map(
				(tileset) =>
					new Promise((resolve) => {
						tileset.image.onload = () => resolve(true)
					})
			)
		)
	}

	public static render(context: CanvasRenderingContext2D, mapData: TileMap) {
		const tileLayer = mapData.layers.find((layer) => layer.name === 'main')

		if (!tileLayer || !tileLayer.visible) {
			return
		}

		const { width, height, data } = tileLayer

		for (let y = 0; y < height; y++) {
			for (let x = 0; x < width; x++) {
				const tileIndex = data[y * width + x] - 1
				if (tileIndex < 0) continue

				for (const tileset of this.tilesets) {
					if (
						tileIndex <
						(tileset.imageWidth / tileset.tileWidth) *
							(tileset.imageHeight / tileset.tileHeight)
					) {
						const cols = tileset.imageWidth / tileset.tileWidth
						const tx = (tileIndex % cols) * tileset.tileWidth
						const ty = Math.floor(tileIndex / cols) * tileset.tileHeight
						context.drawImage(
							tileset.image,
							tx,
							ty,
							tileset.tileWidth,
							tileset.tileHeight,
							x * mapData.tilewidth,
							y * mapData.tileheight,
							mapData.tilewidth,
							mapData.tileheight
						)
						break
					}
				}
			}
		}

		this.renderCoins(context, mapData)
	}

	public static renderCoins(
		context: CanvasRenderingContext2D,
		mapData: TileMap
	) {
		const coinLayer = mapData.layers.find((layer) => layer.name === 'coins')
		if (!coinLayer || !coinLayer.visible) {
			return
		}

		const { width, height, data } = coinLayer

		for (let y = 0; y < height; y++) {
			for (let x = 0; x < width; x++) {
				const tileIndex = data[y * width + x]
				if (tileIndex > 0) {
					const tileset = this.tilesets.find(
						(ts) =>
							tileIndex >= ts.firstgid && tileIndex < ts.firstgid + ts.tilecount
					)
					if (!tileset) continue

					const localTileIndex = tileIndex - tileset.firstgid
					const cols = tileset.imageWidth / tileset.tileWidth
					const tx = (localTileIndex % cols) * tileset.tileWidth
					const ty = Math.floor(localTileIndex / cols) * tileset.tileHeight

					context.drawImage(
						tileset.image,
						tx,
						ty,
						tileset.tileWidth,
						tileset.tileHeight,
						x * mapData.tilewidth,
						y * mapData.tileheight,
						mapData.tilewidth,
						mapData.tileheight
					)
				}
			}
		}
	}
}
