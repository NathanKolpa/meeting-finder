import { Organization } from "../models";

import aaLogoUrl from '../assets/logos/aa.png';
import daLogoUrl from '../assets/logos/da.png';
import cmaLogoUrl from '../assets/logos/cma_resize.webp';
import codaLogoUrl from '../assets/logos/coda.png';

export function getLogoImgUrlByOrg(org: Organization): string {
	switch (org) {
		case "AnonymousAlcoholics":
			return aaLogoUrl;
		case "DebtorsAnonymous":
			return daLogoUrl;
		case "CrystalMethAnonymous":
			return cmaLogoUrl;
		case "CodependentsAnonymous":
			return codaLogoUrl;
	}
}
