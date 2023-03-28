#[doc = "Register `PRIVCFGR2` reader"]
pub struct R(crate::R<PRIVCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCFGR2` writer"]
pub struct W(crate::W<PRIVCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR2_SPEC>;
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
impl From<crate::W<PRIVCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV32` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV32_R = crate::BitReader<bool>;
#[doc = "Field `PRIV32` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV32_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV33` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV33_R = crate::BitReader<bool>;
#[doc = "Field `PRIV33` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV33_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV34` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV34_R = crate::BitReader<bool>;
#[doc = "Field `PRIV34` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV34_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV35` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV35_R = crate::BitReader<bool>;
#[doc = "Field `PRIV35` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV35_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV36` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV36_R = crate::BitReader<bool>;
#[doc = "Field `PRIV36` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV36_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV37` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV37_R = crate::BitReader<bool>;
#[doc = "Field `PRIV37` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV37_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV38` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV38_R = crate::BitReader<bool>;
#[doc = "Field `PRIV38` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV38_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV39` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV39_R = crate::BitReader<bool>;
#[doc = "Field `PRIV39` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV39_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV40` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV40_R = crate::BitReader<bool>;
#[doc = "Field `PRIV40` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV40_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV41` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV41_R = crate::BitReader<bool>;
#[doc = "Field `PRIV41` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV41_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV42` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV42_R = crate::BitReader<bool>;
#[doc = "Field `PRIV42` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV42_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV43` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV43_R = crate::BitReader<bool>;
#[doc = "Field `PRIV43` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV43_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV44` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV44_R = crate::BitReader<bool>;
#[doc = "Field `PRIV44` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV44_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV45` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV45_R = crate::BitReader<bool>;
#[doc = "Field `PRIV45` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV45_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV46` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV46_R = crate::BitReader<bool>;
#[doc = "Field `PRIV46` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV46_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV47` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV47_R = crate::BitReader<bool>;
#[doc = "Field `PRIV47` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV47_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV48` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV48_R = crate::BitReader<bool>;
#[doc = "Field `PRIV48` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV48_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV49` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV49_R = crate::BitReader<bool>;
#[doc = "Field `PRIV49` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV49_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV50` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV50_R = crate::BitReader<bool>;
#[doc = "Field `PRIV50` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV50_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV51` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV51_R = crate::BitReader<bool>;
#[doc = "Field `PRIV51` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV51_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV52` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV52_R = crate::BitReader<bool>;
#[doc = "Field `PRIV52` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV52_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV53` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV53_R = crate::BitReader<bool>;
#[doc = "Field `PRIV53` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV53_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV54` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV54_R = crate::BitReader<bool>;
#[doc = "Field `PRIV54` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV54_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV55` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV55_R = crate::BitReader<bool>;
#[doc = "Field `PRIV55` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV55_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV56` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV56_R = crate::BitReader<bool>;
#[doc = "Field `PRIV56` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV56_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV57` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV57_R = crate::BitReader<bool>;
#[doc = "Field `PRIV57` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV57_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv32(&self) -> PRIV32_R {
        PRIV32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv33(&self) -> PRIV33_R {
        PRIV33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv34(&self) -> PRIV34_R {
        PRIV34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv35(&self) -> PRIV35_R {
        PRIV35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv36(&self) -> PRIV36_R {
        PRIV36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv43(&self) -> PRIV43_R {
        PRIV43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv44(&self) -> PRIV44_R {
        PRIV44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv45(&self) -> PRIV45_R {
        PRIV45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv46(&self) -> PRIV46_R {
        PRIV46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv47(&self) -> PRIV47_R {
        PRIV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv48(&self) -> PRIV48_R {
        PRIV48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv49(&self) -> PRIV49_R {
        PRIV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv50(&self) -> PRIV50_R {
        PRIV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv51(&self) -> PRIV51_R {
        PRIV51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv52(&self) -> PRIV52_R {
        PRIV52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv53(&self) -> PRIV53_R {
        PRIV53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv54(&self) -> PRIV54_R {
        PRIV54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv55(&self) -> PRIV55_R {
        PRIV55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv56(&self) -> PRIV56_R {
        PRIV56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv57(&self) -> PRIV57_R {
        PRIV57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv32(&mut self) -> PRIV32_W<0> {
        PRIV32_W::new(self)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv33(&mut self) -> PRIV33_W<1> {
        PRIV33_W::new(self)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv34(&mut self) -> PRIV34_W<2> {
        PRIV34_W::new(self)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv35(&mut self) -> PRIV35_W<3> {
        PRIV35_W::new(self)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv36(&mut self) -> PRIV36_W<4> {
        PRIV36_W::new(self)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv37(&mut self) -> PRIV37_W<5> {
        PRIV37_W::new(self)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv38(&mut self) -> PRIV38_W<6> {
        PRIV38_W::new(self)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv39(&mut self) -> PRIV39_W<7> {
        PRIV39_W::new(self)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv40(&mut self) -> PRIV40_W<8> {
        PRIV40_W::new(self)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv41(&mut self) -> PRIV41_W<9> {
        PRIV41_W::new(self)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv42(&mut self) -> PRIV42_W<10> {
        PRIV42_W::new(self)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv43(&mut self) -> PRIV43_W<11> {
        PRIV43_W::new(self)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv44(&mut self) -> PRIV44_W<12> {
        PRIV44_W::new(self)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv45(&mut self) -> PRIV45_W<13> {
        PRIV45_W::new(self)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv46(&mut self) -> PRIV46_W<14> {
        PRIV46_W::new(self)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv47(&mut self) -> PRIV47_W<15> {
        PRIV47_W::new(self)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv48(&mut self) -> PRIV48_W<16> {
        PRIV48_W::new(self)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv49(&mut self) -> PRIV49_W<17> {
        PRIV49_W::new(self)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv50(&mut self) -> PRIV50_W<18> {
        PRIV50_W::new(self)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv51(&mut self) -> PRIV51_W<19> {
        PRIV51_W::new(self)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv52(&mut self) -> PRIV52_W<20> {
        PRIV52_W::new(self)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv53(&mut self) -> PRIV53_W<21> {
        PRIV53_W::new(self)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv54(&mut self) -> PRIV54_W<22> {
        PRIV54_W::new(self)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv55(&mut self) -> PRIV55_W<23> {
        PRIV55_W::new(self)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv56(&mut self) -> PRIV56_W<24> {
        PRIV56_W::new(self)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv57(&mut self) -> PRIV57_W<25> {
        PRIV57_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI privilege configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr2](index.html) module"]
pub struct PRIVCFGR2_SPEC;
impl crate::RegisterSpec for PRIVCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcfgr2::R](R) reader structure"]
impl crate::Readable for PRIVCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcfgr2::W](W) writer structure"]
impl crate::Writable for PRIVCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIVCFGR2 to value 0"]
impl crate::Resettable for PRIVCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
