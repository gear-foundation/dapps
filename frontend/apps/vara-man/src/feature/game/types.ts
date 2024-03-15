export type TileMap = {
	compressionlevel: number
	height: number
	infinite: boolean
	layers: Array<{
		data: number[]
		height: number
		id: number
		name: string
		opacity: number
		type: string
		visible: boolean
		width: number
		x: number
		y: number
	}>
	nextlayerid: number
	nextobjectid: number
	orientation: string
	renderorder: string
	tiledversion: string
	tileheight: number
	tilesets: Array<{
		columns: number
		firstgid: number
		image: string
		imageheight: number
		imagewidth: number
		margin: number
		name: string
		spacing: number
		tilecount: number
		tileheight: number
		tilewidth: number
	}>
	tilewidth: number
	type: string
	version: string | number
	width: number
}
