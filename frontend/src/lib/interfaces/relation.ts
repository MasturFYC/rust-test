export type iRelationBase = {
   id: number;
   city: string;
   street?: string;
   phone?: string;
   isSpecial: boolean;
   photo?: string;
}

export type iRelationProp = iRelationBase & {
    text: string;
}

export type iRelation = iRelationBase & {
   name: string;
   isActive: boolean;
   relationType: string[];
   createdAt: string;
   updatedAt: string;
}

export type RelationTypeWIthID = {
	id: string;
	text: string;
}
