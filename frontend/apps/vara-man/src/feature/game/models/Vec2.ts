export class Vec2 {
	x: number
	y: number

	constructor(x = 0, y = 0) {
		this.x = x
		this.y = y
	}

	add(v: Vec2): Vec2 {
		this.x += v.x
		this.y += v.y
		return this
	}

	sub(v: Vec2): Vec2 {
		this.x -= v.x
		this.y -= v.y
		return this
	}

	mult(s: number): Vec2 {
		this.x *= s
		this.y *= s
		return this
	}

	mag(): number {
		return Math.sqrt(this.x * this.x + this.y * this.y)
	}

	norm(): Vec2 {
		const m = this.mag()
		if (m > 0) {
			this.x /= m
			this.y /= m
		}
		return this
	}

	rotate(a: number): Vec2 {
		const sina = Math.sin(a)
		const cosa = Math.cos(a)
		const rx = this.x * cosa - this.y * sina
		const ry = this.x * sina + this.y * cosa
		this.x = rx
		this.y = ry
		return this
	}

	copy(): Vec2 {
		return new Vec2(this.x, this.y)
	}

	set(x: number, y: number): void {
		this.x = x
		this.y = y
	}

	scale(scalar: number): Vec2 {
		return new Vec2(this.x * scalar, this.y * scalar)
	}

	static add(v1: Vec2, v2: Vec2): Vec2 {
		return new Vec2(v1.x + v2.x, v1.y + v2.y)
	}

	static distance(v1: Vec2, v2: Vec2): number {
		const dx = v2.x - v1.x
		const dy = v2.y - v1.y
		return Math.sqrt(dx * dx + dy * dy)
	}

	distanceTo(v: Vec2): number {
		const dx = v.x - this.x
		const dy = v.y - this.y
		return Math.sqrt(dx * dx + dy * dy)
	}

	static subtract(v1: Vec2, v2: Vec2): Vec2 {
		return new Vec2(v1.x - v2.x, v1.y - v2.y)
	}
}
