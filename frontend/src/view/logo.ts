import { Organization } from "../models";

import aaLogoUrl from '../assets/logos/aa.png';
import daLogoUrl from '../assets/logos/da.png';

export function getLogoImgUrlByOrg(org: Organization): string {
	switch (org) {
		case "AnonymousAlcoholics":
			return aaLogoUrl;
		case "DebtorsAnonymous":
			return daLogoUrl;
	}
}
