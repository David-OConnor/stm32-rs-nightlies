#[doc = "Register `LPTIM_DIER_intput` reader"]
pub struct R(crate::R<LPTIM_DIER_INTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_DIER_INTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_DIER_INTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_DIER_INTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_DIER_intput` writer"]
pub struct W(crate::W<LPTIM_DIER_INTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_DIER_INTPUT_SPEC>;
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
impl From<crate::W<LPTIM_DIER_INTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_DIER_INTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1IE` reader - Capture/compare 1 interrupt enable"]
pub type CC1IE_R = crate::BitReader<bool>;
#[doc = "Field `CC1IE` writer - Capture/compare 1 interrupt enable"]
pub type CC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ARRMIE_R = crate::BitReader<bool>;
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ARRMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_R = crate::BitReader<bool>;
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_R = crate::BitReader<bool>;
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_R = crate::BitReader<bool>;
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_R = crate::BitReader<bool>;
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `UEIE` reader - Update event interrupt enable"]
pub type UEIE_R = crate::BitReader<bool>;
#[doc = "Field `UEIE` writer - Update event interrupt enable"]
pub type UEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `REPOKIE` reader - Repetition register update OK interrupt Enable"]
pub type REPOKIE_R = crate::BitReader<bool>;
#[doc = "Field `REPOKIE` writer - Repetition register update OK interrupt Enable"]
pub type REPOKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC2IE` reader - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_R = crate::BitReader<bool>;
#[doc = "Field `CC2IE` writer - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC1OIE` reader - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1OIE_R = crate::BitReader<bool>;
#[doc = "Field `CC1OIE` writer - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC2OIE` reader - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2OIE_R = crate::BitReader<bool>;
#[doc = "Field `CC2OIE` writer - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC1DE` reader - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1DE_R = crate::BitReader<bool>;
#[doc = "Field `CC1DE` writer - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `UEDE` reader - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_R = crate::BitReader<bool>;
#[doc = "Field `UEDE` writer - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC2DE` reader - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2DE_R = crate::BitReader<bool>;
#[doc = "Field `CC2DE` writer - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_DIER_INTPUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc1oie(&self) -> CC1OIE_R {
        CC1OIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2oie(&self) -> CC2OIE_R {
        CC2OIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn uede(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<0> {
        CC1IE_W::new(self)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<1> {
        ARRMIE_W::new(self)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<2> {
        EXTTRIGIE_W::new(self)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<4> {
        ARROKIE_W::new(self)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<5> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<6> {
        DOWNIE_W::new(self)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ueie(&mut self) -> UEIE_W<7> {
        UEIE_W::new(self)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn repokie(&mut self) -> REPOKIE_W<8> {
        REPOKIE_W::new(self)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<9> {
        CC2IE_W::new(self)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc1oie(&mut self) -> CC1OIE_W<12> {
        CC1OIE_W::new(self)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2oie(&mut self) -> CC2OIE_W<13> {
        CC2OIE_W::new(self)
    }
    #[doc = "Bit 16 - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<16> {
        CC1DE_W::new(self)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn uede(&mut self) -> UEDE_W<23> {
        UEDE_W::new(self)
    }
    #[doc = "Bit 25 - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CC2DE_W<25> {
        CC2DE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_dier_intput](index.html) module"]
pub struct LPTIM_DIER_INTPUT_SPEC;
impl crate::RegisterSpec for LPTIM_DIER_INTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_dier_intput::R](R) reader structure"]
impl crate::Readable for LPTIM_DIER_INTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_dier_intput::W](W) writer structure"]
impl crate::Writable for LPTIM_DIER_INTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_DIER_intput to value 0"]
impl crate::Resettable for LPTIM_DIER_INTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
