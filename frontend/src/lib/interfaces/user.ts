
export type iUserLogin = {
	email: string,
	password: string
}

export type iCurrentUser = {
	id: string,
	name: string,
	email: string,
	photo: string,
	role: string,
	verified: boolean,
	updatedAt: string,
	createdAt: string
}

export type iUser = {
	id: number,

	roles: string[],
	// tokenType: string;
	token: string
	// refreshToken: string;
}
