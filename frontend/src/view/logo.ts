import aaLogoUrl from '../assets/logos/aa.png';
import {Organization} from "../models";

export function getLogoImgUrlByOrg(org: Organization): string {
    switch (org) {
        case "AnonymousAlcoholics":
            return aaLogoUrl;
    }
}