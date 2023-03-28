#[doc = "Register `IMR2` reader"]
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR2` writer"]
pub struct W(crate::W<IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR2_SPEC>;
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
impl From<crate::W<IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM37` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM37_R = crate::BitReader<bool>;
#[doc = "Field `IM37` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM37_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM38` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM38_R = crate::BitReader<bool>;
#[doc = "Field `IM38` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM38_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM39` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM39_R = crate::BitReader<bool>;
#[doc = "Field `IM39` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM39_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM40` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM40_R = crate::BitReader<bool>;
#[doc = "Field `IM40` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM40_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM41` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM41_R = crate::BitReader<bool>;
#[doc = "Field `IM41` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM41_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM42` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM42_R = crate::BitReader<bool>;
#[doc = "Field `IM42` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM42_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM47` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM47_R = crate::BitReader<bool>;
#[doc = "Field `IM47` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM47_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM49` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM49_R = crate::BitReader<bool>;
#[doc = "Field `IM49` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM49_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM50` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM50_R = crate::BitReader<bool>;
#[doc = "Field `IM50` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM50_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM53` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM53_R = crate::BitReader<bool>;
#[doc = "Field `IM53` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM53_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im47(&self) -> IM47_R {
        IM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im49(&self) -> IM49_R {
        IM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im50(&self) -> IM50_R {
        IM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im53(&self) -> IM53_R {
        IM53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> IM37_W<5> {
        IM37_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> IM38_W<6> {
        IM38_W::new(self)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im39(&mut self) -> IM39_W<7> {
        IM39_W::new(self)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> IM40_W<8> {
        IM40_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> IM41_W<9> {
        IM41_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<10> {
        IM42_W::new(self)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im47(&mut self) -> IM47_W<15> {
        IM47_W::new(self)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im49(&mut self) -> IM49_W<17> {
        IM49_W::new(self)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im50(&mut self) -> IM50_W<18> {
        IM50_W::new(self)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im53(&mut self) -> IM53_W<21> {
        IM53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](index.html) module"]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr2::R](R) reader structure"]
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr2::W](W) writer structure"]
impl crate::Writable for IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0x00db_bfff"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x00db_bfff;
}
