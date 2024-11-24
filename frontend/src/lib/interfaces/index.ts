export * from "./product";
export * from "./relation";
export * from "./order";
export * from "./stock";
export * from "./user";

export const baseURL = import.meta.env.VITE_API_URL;
export const credential_include: RequestCredentials = "include";

export type CardData = {
  code: string;
  prefix: number;
  start: number;
  end: number;
  year: number;
};

export type Property = {
  id: number;
  name: string;
};

export type iCPOCategory = Property & {
  image: string;
};

export type iCPO = {
  category: number;
  sinceFirst: number;
  sinceLast: number;
  prefix: number;
  start: number;
  count: number;
  scale: number;
  updatedAt: string;
};

export type School = Property & {
  nilai: number[];
};

export type Kecamatan2 = Property & {
  schools: School[];
  isSelected?: boolean;
};

export type FileInfo = {
  name: string;
  url: string;
  isLoading?: boolean;
};

export type iMemberCard = {
  id: number;
  name: string;
  numberStart: number;
  numberEnd: number;
  updatedAt: number;
  since: number;
};

export type PDAM = {
  slid: number;
  name: string;
  address: string;
  branch: string;
  // selected?: boolean;
};

export type label103Setting = {
  col: number;
  row: number;
  labelWidth: number;
  labelHeight: number;
  pageWidth: number; //paper
  pageHeight: number; //paper
  horizontalPitch: number;
  verticalPitch: number;
  addBracket: boolean;
  fontSize: number;
  fontName: string;
  lineHeight: number;
  addDi: boolean;
};

export type Label103 = {
  id: number;
  name1: string;
  name2?: string;
  job?: string;
  address: string;
  city: string;
};

export type Mdta = {
  id: number;
  kecamatanId: number;
  num: number;
  name: string;
  nilai: number[];
};

export type RekapMdta = {
  id: number;
  kecamatanId: number;
  num: number;
  name: string;
  nilai: number;
};

export type Kecamatan = Property & {
  id: number;
  name: string;
  isSelected?: boolean;
};

export type Cipto = {
  prefix: number;
  start: number;
  count: number;
  since: number;
  scale: number;
  updatedAt: string;
};

export type iMusaTani = Cipto;

export type iLogo = {
  id: number;
  title: string;
  author: string;
  descriptions: string;
  resourceName: string;
  previewName: string;
  isProtected: boolean;
};

export type iProperty = {
  id: string;
  text: string;
};

export type iPropertyID = {
  id: number;
  text: string;
};
