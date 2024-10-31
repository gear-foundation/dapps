import { Level } from '@/app/utils'
import EasyMap from '../assets/map/map-easy.json'
import MediumMap from '../assets/map/map-medium.json'
import HardMap from '../assets/map/map-hard.json'
import { TileMap } from '../types'

const maps = {
	Easy: EasyMap,
	Medium: MediumMap,
	Hard: HardMap,
}

export const findMapLevel = (level: Level): TileMap => {
	const map = maps[level]

	if (!map) {
		throw new Error(`Map for level "${level}" not found.`)
	}

	return JSON.parse(JSON.stringify(map))
}
