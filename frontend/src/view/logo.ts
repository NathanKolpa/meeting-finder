import { Organization } from "../models";

import aaLogoUrl from '../assets/logos/aa.png';
import daLogoUrl from '../assets/logos/da.png';
import cmaLogoUrl from '../assets/logos/cma_resize.webp';

export function getLogoImgUrlByOrg(org: Organization): string {
	switch (org) {
		case "AnonymousAlcoholics":
			return aaLogoUrl;
		case "DebtorsAnonymous":
			return daLogoUrl;
		case "CrystalMethAnonymous":
			return cmaLogoUrl;
	}
}
