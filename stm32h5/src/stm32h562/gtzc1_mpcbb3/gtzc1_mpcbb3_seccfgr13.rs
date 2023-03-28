#[doc = "Register `GTZC1_MPCBB3_SECCFGR13` reader"]
pub struct R(crate::R<GTZC1_MPCBB3_SECCFGR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_MPCBB3_SECCFGR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_MPCBB3_SECCFGR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_MPCBB3_SECCFGR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_MPCBB3_SECCFGR13` writer"]
pub struct W(crate::W<GTZC1_MPCBB3_SECCFGR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_MPCBB3_SECCFGR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTZC1_MPCBB3_SECCFGR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_MPCBB3_SECCFGR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC0_R = crate::BitReader<bool>;
#[doc = "Field `SEC0` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC1` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC1_R = crate::BitReader<bool>;
#[doc = "Field `SEC1` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC2` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC2_R = crate::BitReader<bool>;
#[doc = "Field `SEC2` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC3` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC3_R = crate::BitReader<bool>;
#[doc = "Field `SEC3` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC4` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC4_R = crate::BitReader<bool>;
#[doc = "Field `SEC4` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC5` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC5_R = crate::BitReader<bool>;
#[doc = "Field `SEC5` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC6` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC6_R = crate::BitReader<bool>;
#[doc = "Field `SEC6` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC7` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC7_R = crate::BitReader<bool>;
#[doc = "Field `SEC7` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC8` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC8_R = crate::BitReader<bool>;
#[doc = "Field `SEC8` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC9` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC9_R = crate::BitReader<bool>;
#[doc = "Field `SEC9` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC10` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC10_R = crate::BitReader<bool>;
#[doc = "Field `SEC10` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC11` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC11_R = crate::BitReader<bool>;
#[doc = "Field `SEC11` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC12` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC12_R = crate::BitReader<bool>;
#[doc = "Field `SEC12` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC13` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC13_R = crate::BitReader<bool>;
#[doc = "Field `SEC13` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC14` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC14_R = crate::BitReader<bool>;
#[doc = "Field `SEC14` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC15` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC15_R = crate::BitReader<bool>;
#[doc = "Field `SEC15` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC16` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC16_R = crate::BitReader<bool>;
#[doc = "Field `SEC16` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC17` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC17_R = crate::BitReader<bool>;
#[doc = "Field `SEC17` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC18` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC18_R = crate::BitReader<bool>;
#[doc = "Field `SEC18` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC19` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC19_R = crate::BitReader<bool>;
#[doc = "Field `SEC19` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC20` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC20_R = crate::BitReader<bool>;
#[doc = "Field `SEC20` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC21` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC21_R = crate::BitReader<bool>;
#[doc = "Field `SEC21` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC22` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC22_R = crate::BitReader<bool>;
#[doc = "Field `SEC22` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC23` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC23_R = crate::BitReader<bool>;
#[doc = "Field `SEC23` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC24` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC24_R = crate::BitReader<bool>;
#[doc = "Field `SEC24` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC25` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC25_R = crate::BitReader<bool>;
#[doc = "Field `SEC25` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC26` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC26_R = crate::BitReader<bool>;
#[doc = "Field `SEC26` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC27` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC27_R = crate::BitReader<bool>;
#[doc = "Field `SEC27` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC28` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC28_R = crate::BitReader<bool>;
#[doc = "Field `SEC28` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC29` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC29_R = crate::BitReader<bool>;
#[doc = "Field `SEC29` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC30` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC30_R = crate::BitReader<bool>;
#[doc = "Field `SEC30` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
#[doc = "Field `SEC31` reader - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC31_R = crate::BitReader<bool>;
#[doc = "Field `SEC31` writer - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type SEC31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB3_SECCFGR13_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec16(&self) -> SEC16_R {
        SEC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec17(&self) -> SEC17_R {
        SEC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec18(&self) -> SEC18_R {
        SEC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec19(&self) -> SEC19_R {
        SEC19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec20(&self) -> SEC20_R {
        SEC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec21(&self) -> SEC21_R {
        SEC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec22(&self) -> SEC22_R {
        SEC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec23(&self) -> SEC23_R {
        SEC23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec24(&self) -> SEC24_R {
        SEC24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec25(&self) -> SEC25_R {
        SEC25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec26(&self) -> SEC26_R {
        SEC26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec27(&self) -> SEC27_R {
        SEC27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec28(&self) -> SEC28_R {
        SEC28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec29(&self) -> SEC29_R {
        SEC29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec30(&self) -> SEC30_R {
        SEC30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn sec31(&self) -> SEC31_R {
        SEC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<0> {
        SEC0_W::new(self)
    }
    #[doc = "Bit 1 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<1> {
        SEC1_W::new(self)
    }
    #[doc = "Bit 2 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<2> {
        SEC2_W::new(self)
    }
    #[doc = "Bit 3 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<3> {
        SEC3_W::new(self)
    }
    #[doc = "Bit 4 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<4> {
        SEC4_W::new(self)
    }
    #[doc = "Bit 5 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<5> {
        SEC5_W::new(self)
    }
    #[doc = "Bit 6 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<6> {
        SEC6_W::new(self)
    }
    #[doc = "Bit 7 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<7> {
        SEC7_W::new(self)
    }
    #[doc = "Bit 8 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec8(&mut self) -> SEC8_W<8> {
        SEC8_W::new(self)
    }
    #[doc = "Bit 9 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec9(&mut self) -> SEC9_W<9> {
        SEC9_W::new(self)
    }
    #[doc = "Bit 10 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<10> {
        SEC10_W::new(self)
    }
    #[doc = "Bit 11 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec11(&mut self) -> SEC11_W<11> {
        SEC11_W::new(self)
    }
    #[doc = "Bit 12 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec12(&mut self) -> SEC12_W<12> {
        SEC12_W::new(self)
    }
    #[doc = "Bit 13 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec13(&mut self) -> SEC13_W<13> {
        SEC13_W::new(self)
    }
    #[doc = "Bit 14 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec14(&mut self) -> SEC14_W<14> {
        SEC14_W::new(self)
    }
    #[doc = "Bit 15 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec15(&mut self) -> SEC15_W<15> {
        SEC15_W::new(self)
    }
    #[doc = "Bit 16 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec16(&mut self) -> SEC16_W<16> {
        SEC16_W::new(self)
    }
    #[doc = "Bit 17 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec17(&mut self) -> SEC17_W<17> {
        SEC17_W::new(self)
    }
    #[doc = "Bit 18 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec18(&mut self) -> SEC18_W<18> {
        SEC18_W::new(self)
    }
    #[doc = "Bit 19 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec19(&mut self) -> SEC19_W<19> {
        SEC19_W::new(self)
    }
    #[doc = "Bit 20 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec20(&mut self) -> SEC20_W<20> {
        SEC20_W::new(self)
    }
    #[doc = "Bit 21 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec21(&mut self) -> SEC21_W<21> {
        SEC21_W::new(self)
    }
    #[doc = "Bit 22 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec22(&mut self) -> SEC22_W<22> {
        SEC22_W::new(self)
    }
    #[doc = "Bit 23 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec23(&mut self) -> SEC23_W<23> {
        SEC23_W::new(self)
    }
    #[doc = "Bit 24 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec24(&mut self) -> SEC24_W<24> {
        SEC24_W::new(self)
    }
    #[doc = "Bit 25 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec25(&mut self) -> SEC25_W<25> {
        SEC25_W::new(self)
    }
    #[doc = "Bit 26 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec26(&mut self) -> SEC26_W<26> {
        SEC26_W::new(self)
    }
    #[doc = "Bit 27 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec27(&mut self) -> SEC27_W<27> {
        SEC27_W::new(self)
    }
    #[doc = "Bit 28 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec28(&mut self) -> SEC28_W<28> {
        SEC28_W::new(self)
    }
    #[doc = "Bit 29 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec29(&mut self) -> SEC29_W<29> {
        SEC29_W::new(self)
    }
    #[doc = "Bit 30 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec30(&mut self) -> SEC30_W<30> {
        SEC30_W::new(self)
    }
    #[doc = "Bit 31 - Security configuration for block y Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn sec31(&mut self) -> SEC31_W<31> {
        SEC31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 13 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_mpcbb3_seccfgr13](index.html) module"]
pub struct GTZC1_MPCBB3_SECCFGR13_SPEC;
impl crate::RegisterSpec for GTZC1_MPCBB3_SECCFGR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_mpcbb3_seccfgr13::R](R) reader structure"]
impl crate::Readable for GTZC1_MPCBB3_SECCFGR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_mpcbb3_seccfgr13::W](W) writer structure"]
impl crate::Writable for GTZC1_MPCBB3_SECCFGR13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_MPCBB3_SECCFGR13 to value 0xffff_ffff"]
impl crate::Resettable for GTZC1_MPCBB3_SECCFGR13_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
