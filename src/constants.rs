/*
 * ANISE Toolkit
 * Copyright (C) 2021-2022 Christopher Rabotin <christop&her.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */

pub mod celestial_objects {
    /// Solar System barycenter is fixed to a hash of zero.
    pub const SOLAR_SYSTEM_BARYCENTER: u32 = 0;
    /// Source bytes: `Mercury`
    pub const MERCURY: u32 = 753059387;
    /// Source bytes: `Venus`
    pub const VENUS: u32 = 2760147288;
    /// Source bytes: `Earth-Moon Barycenter`
    pub const EARTH_MOON_BARYCENTER: u32 = 46073813;
    /// Source bytes: `Mars Barycenter`
    pub const MARS_BARYCENTER: u32 = 1223981629;
    /// Source bytes: `Jupyter Barycenter`
    pub const JUPITER_BARYCENTER: u32 = 2905700239;
    /// Source bytes: `Saturn Barycenter`
    pub const SATURN_BARYCENTER: u32 = 2400246439;
    /// Source bytes: `Uranus Barycenter`
    pub const URANUS_BARYCENTER: u32 = 1449143244;
    /// Source bytes: `Neptune Barycenter`
    pub const NEPTUNE_BARYCENTER: u32 = 199396881;
    /// Source bytes: `Pluto Barycenter`
    pub const PLUTO_BARYCENTER: u32 = 1544737610;
    /// Source bytes: `Sun`
    pub const SUN: u32 = 1777960983;
    /// Source bytes: `Luna`
    pub const LUNA: u32 = 1668777413;
    /// Source bytes: `Earth`
    pub const EARTH: u32 = 2330221028;

    pub const fn hash_celestial_name<'a>(hash: u32) -> Option<&'a str> {
        match hash {
            MERCURY => Some("Mercury"),
            VENUS => Some("Venus"),
            EARTH_MOON_BARYCENTER => Some("Earth-Moon Barycenter"),
            MARS_BARYCENTER => Some("Mars Barycenter"),
            JUPITER_BARYCENTER => Some("Jupiter Barycenter"),
            SATURN_BARYCENTER => Some("Saturn Barycenter"),
            URANUS_BARYCENTER => Some("Uranus Barycenter"),
            NEPTUNE_BARYCENTER => Some("Neptune Barycenter"),
            PLUTO_BARYCENTER => Some("Pluto Barycenter"),
            SUN => Some("Sun"),
            LUNA => Some("Luna"),
            EARTH => Some("Earth"),
            _ => None,
        }
    }
}

pub mod orientations {
    /// Source bytes: `J2000`
    pub const J2000: u32 = 1404527632;

    pub const fn hash_orientation_name<'a>(hash: u32) -> Option<&'a str> {
        match hash {
            J2000 => Some("J2000"),
            _ => None,
        }
    }
}

pub mod frames {
    use crate::frame::Frame;

    use super::{celestial_objects::*, orientations::J2000};

    pub const MERCURY_J2000: Frame = Frame::from_ephem_orient_hashes(MERCURY, J2000);
    pub const VENUS_J2000: Frame = Frame::from_ephem_orient_hashes(VENUS, J2000);
    pub const EARTH_MOON_BARYCENTER_J2000: Frame =
        Frame::from_ephem_orient_hashes(EARTH_MOON_BARYCENTER, J2000);
    pub const MARS_BARYCENTER_J2000: Frame =
        Frame::from_ephem_orient_hashes(MARS_BARYCENTER, J2000);
    pub const JUPITER_BARYCENTER_J2000: Frame =
        Frame::from_ephem_orient_hashes(JUPITER_BARYCENTER, J2000);
    pub const SATURN_BARYCENTER_J2000: Frame =
        Frame::from_ephem_orient_hashes(SATURN_BARYCENTER, J2000);
    pub const URANUS_BARYCENTER_J2000: Frame =
        Frame::from_ephem_orient_hashes(URANUS_BARYCENTER, J2000);
    pub const NEPTUNE_BARYCENTER_J2000: Frame =
        Frame::from_ephem_orient_hashes(NEPTUNE_BARYCENTER, J2000);
    pub const PLUTO_BARYCENTER_J2000: Frame =
        Frame::from_ephem_orient_hashes(PLUTO_BARYCENTER, J2000);
    pub const SUN_J2000: Frame = Frame::from_ephem_orient_hashes(SUN, J2000);
    pub const LUNA_J2000: Frame = Frame::from_ephem_orient_hashes(LUNA, J2000);
    pub const EARTH_J2000: Frame = Frame::from_ephem_orient_hashes(EARTH, J2000);
}
