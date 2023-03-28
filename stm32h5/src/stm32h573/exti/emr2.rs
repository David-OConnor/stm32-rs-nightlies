#[doc = "Register `EMR2` reader"]
pub struct R(crate::R<EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR2` writer"]
pub struct W(crate::W<EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR2_SPEC>;
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
impl From<crate::W<EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM32` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM32_R = crate::BitReader<bool>;
#[doc = "Field `EM32` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM32_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM33` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM33_R = crate::BitReader<bool>;
#[doc = "Field `EM33` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM33_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM34` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM34_R = crate::BitReader<bool>;
#[doc = "Field `EM34` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM34_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM35` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM35_R = crate::BitReader<bool>;
#[doc = "Field `EM35` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM35_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM36` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM36_R = crate::BitReader<bool>;
#[doc = "Field `EM36` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM36_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM37` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_R = crate::BitReader<bool>;
#[doc = "Field `EM37` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM38` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM38_R = crate::BitReader<bool>;
#[doc = "Field `EM38` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM38_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM39` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM39_R = crate::BitReader<bool>;
#[doc = "Field `EM39` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM39_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM40` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM40_R = crate::BitReader<bool>;
#[doc = "Field `EM40` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM40_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM41` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM41_R = crate::BitReader<bool>;
#[doc = "Field `EM41` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM41_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM42` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM42_R = crate::BitReader<bool>;
#[doc = "Field `EM42` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM42_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM43` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM43_R = crate::BitReader<bool>;
#[doc = "Field `EM43` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM43_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM44` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM44_R = crate::BitReader<bool>;
#[doc = "Field `EM44` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM44_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM45` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM45_R = crate::BitReader<bool>;
#[doc = "Field `EM45` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM45_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM46` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM46_R = crate::BitReader<bool>;
#[doc = "Field `EM46` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM46_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM47` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM47_R = crate::BitReader<bool>;
#[doc = "Field `EM47` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM47_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM48` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM48_R = crate::BitReader<bool>;
#[doc = "Field `EM48` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM48_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM49` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM49_R = crate::BitReader<bool>;
#[doc = "Field `EM49` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM49_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM50` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM50_R = crate::BitReader<bool>;
#[doc = "Field `EM50` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM50_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM51` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM51_R = crate::BitReader<bool>;
#[doc = "Field `EM51` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM51_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM52` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM52_R = crate::BitReader<bool>;
#[doc = "Field `EM52` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM52_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM53` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM53_R = crate::BitReader<bool>;
#[doc = "Field `EM53` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM53_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM54` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM54_R = crate::BitReader<bool>;
#[doc = "Field `EM54` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM54_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM55` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM55_R = crate::BitReader<bool>;
#[doc = "Field `EM55` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM55_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM56` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM56_R = crate::BitReader<bool>;
#[doc = "Field `EM56` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM56_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM57` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM57_R = crate::BitReader<bool>;
#[doc = "Field `EM57` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM57_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em36(&self) -> EM36_R {
        EM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em38(&self) -> EM38_R {
        EM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em39(&self) -> EM39_R {
        EM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em43(&self) -> EM43_R {
        EM43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em44(&self) -> EM44_R {
        EM44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em45(&self) -> EM45_R {
        EM45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em46(&self) -> EM46_R {
        EM46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em47(&self) -> EM47_R {
        EM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em48(&self) -> EM48_R {
        EM48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em49(&self) -> EM49_R {
        EM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em50(&self) -> EM50_R {
        EM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em51(&self) -> EM51_R {
        EM51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em52(&self) -> EM52_R {
        EM52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em53(&self) -> EM53_R {
        EM53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em54(&self) -> EM54_R {
        EM54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em55(&self) -> EM55_R {
        EM55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em56(&self) -> EM56_R {
        EM56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em57(&self) -> EM57_R {
        EM57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em32(&mut self) -> EM32_W<0> {
        EM32_W::new(self)
    }
    #[doc = "Bit 1 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em33(&mut self) -> EM33_W<1> {
        EM33_W::new(self)
    }
    #[doc = "Bit 2 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em34(&mut self) -> EM34_W<2> {
        EM34_W::new(self)
    }
    #[doc = "Bit 3 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em35(&mut self) -> EM35_W<3> {
        EM35_W::new(self)
    }
    #[doc = "Bit 4 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em36(&mut self) -> EM36_W<4> {
        EM36_W::new(self)
    }
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em37(&mut self) -> EM37_W<5> {
        EM37_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em38(&mut self) -> EM38_W<6> {
        EM38_W::new(self)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em39(&mut self) -> EM39_W<7> {
        EM39_W::new(self)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<8> {
        EM40_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<9> {
        EM41_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em42(&mut self) -> EM42_W<10> {
        EM42_W::new(self)
    }
    #[doc = "Bit 11 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em43(&mut self) -> EM43_W<11> {
        EM43_W::new(self)
    }
    #[doc = "Bit 12 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em44(&mut self) -> EM44_W<12> {
        EM44_W::new(self)
    }
    #[doc = "Bit 13 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em45(&mut self) -> EM45_W<13> {
        EM45_W::new(self)
    }
    #[doc = "Bit 14 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em46(&mut self) -> EM46_W<14> {
        EM46_W::new(self)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em47(&mut self) -> EM47_W<15> {
        EM47_W::new(self)
    }
    #[doc = "Bit 16 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em48(&mut self) -> EM48_W<16> {
        EM48_W::new(self)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em49(&mut self) -> EM49_W<17> {
        EM49_W::new(self)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em50(&mut self) -> EM50_W<18> {
        EM50_W::new(self)
    }
    #[doc = "Bit 19 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em51(&mut self) -> EM51_W<19> {
        EM51_W::new(self)
    }
    #[doc = "Bit 20 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em52(&mut self) -> EM52_W<20> {
        EM52_W::new(self)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em53(&mut self) -> EM53_W<21> {
        EM53_W::new(self)
    }
    #[doc = "Bit 22 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em54(&mut self) -> EM54_W<22> {
        EM54_W::new(self)
    }
    #[doc = "Bit 23 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em55(&mut self) -> EM55_W<23> {
        EM55_W::new(self)
    }
    #[doc = "Bit 24 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em56(&mut self) -> EM56_W<24> {
        EM56_W::new(self)
    }
    #[doc = "Bit 25 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em57(&mut self) -> EM57_W<25> {
        EM57_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with event mask register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](index.html) module"]
pub struct EMR2_SPEC;
impl crate::RegisterSpec for EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr2::R](R) reader structure"]
impl crate::Readable for EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr2::W](W) writer structure"]
impl crate::Writable for EMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
