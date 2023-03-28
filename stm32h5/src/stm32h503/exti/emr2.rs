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
#[doc = "Field `EM37` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_R = crate::BitReader<bool>;
#[doc = "Field `EM37` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM38` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM38_R = crate::BitReader<bool>;
#[doc = "Field `EM38` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM38_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM39` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM39_R = crate::BitReader<bool>;
#[doc = "Field `EM39` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM39_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM40` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM40_R = crate::BitReader<bool>;
#[doc = "Field `EM40` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM40_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM41` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM41_R = crate::BitReader<bool>;
#[doc = "Field `EM41` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM41_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM42` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM42_R = crate::BitReader<bool>;
#[doc = "Field `EM42` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM42_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM47` reader - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM47_R = crate::BitReader<bool>;
#[doc = "Field `EM47` writer - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM47_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM49` reader - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM49_R = crate::BitReader<bool>;
#[doc = "Field `EM49` writer - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM49_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM50` reader - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM50_R = crate::BitReader<bool>;
#[doc = "Field `EM50` writer - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM50_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM53` reader - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM53_R = crate::BitReader<bool>;
#[doc = "Field `EM53` writer - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM53_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em38(&self) -> EM38_R {
        EM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em39(&self) -> EM39_R {
        EM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em47(&self) -> EM47_R {
        EM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em49(&self) -> EM49_R {
        EM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em50(&self) -> EM50_R {
        EM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em53(&self) -> EM53_R {
        EM53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em37(&mut self) -> EM37_W<5> {
        EM37_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em38(&mut self) -> EM38_W<6> {
        EM38_W::new(self)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em39(&mut self) -> EM39_W<7> {
        EM39_W::new(self)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<8> {
        EM40_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<9> {
        EM41_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em42(&mut self) -> EM42_W<10> {
        EM42_W::new(self)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em47(&mut self) -> EM47_W<15> {
        EM47_W::new(self)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em49(&mut self) -> EM49_W<17> {
        EM49_W::new(self)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em50(&mut self) -> EM50_W<18> {
        EM50_W::new(self)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em53(&mut self) -> EM53_W<21> {
        EM53_W::new(self)
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
#[doc = "`reset()` method sets EMR2 to value 0x00db_bfff"]
impl crate::Resettable for EMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x00db_bfff;
}
