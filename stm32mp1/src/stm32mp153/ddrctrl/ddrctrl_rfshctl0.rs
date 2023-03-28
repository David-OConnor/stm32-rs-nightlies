#[doc = "Register `DDRCTRL_RFSHCTL0` reader"]
pub struct R(crate::R<DDRCTRL_RFSHCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_RFSHCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_RFSHCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_RFSHCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_RFSHCTL0` writer"]
pub struct W(crate::W<DDRCTRL_RFSHCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_RFSHCTL0_SPEC>;
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
impl From<crate::W<DDRCTRL_RFSHCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_RFSHCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_BANK_REFRESH` reader - PER_BANK_REFRESH"]
pub type PER_BANK_REFRESH_R = crate::BitReader<bool>;
#[doc = "Field `PER_BANK_REFRESH` writer - PER_BANK_REFRESH"]
pub type PER_BANK_REFRESH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_RFSHCTL0_SPEC, bool, O>;
#[doc = "Field `REFRESH_BURST` reader - REFRESH_BURST"]
pub type REFRESH_BURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFRESH_BURST` writer - REFRESH_BURST"]
pub type REFRESH_BURST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_RFSHCTL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `REFRESH_TO_X32` reader - REFRESH_TO_X32"]
pub type REFRESH_TO_X32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFRESH_TO_X32` writer - REFRESH_TO_X32"]
pub type REFRESH_TO_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_RFSHCTL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `REFRESH_MARGIN` reader - REFRESH_MARGIN"]
pub type REFRESH_MARGIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFRESH_MARGIN` writer - REFRESH_MARGIN"]
pub type REFRESH_MARGIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_RFSHCTL0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    pub fn per_bank_refresh(&self) -> PER_BANK_REFRESH_R {
        PER_BANK_REFRESH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    pub fn refresh_burst(&self) -> REFRESH_BURST_R {
        REFRESH_BURST_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    pub fn refresh_to_x32(&self) -> REFRESH_TO_X32_R {
        REFRESH_TO_X32_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    pub fn refresh_margin(&self) -> REFRESH_MARGIN_R {
        REFRESH_MARGIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn per_bank_refresh(&mut self) -> PER_BANK_REFRESH_W<2> {
        PER_BANK_REFRESH_W::new(self)
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    #[must_use]
    pub fn refresh_burst(&mut self) -> REFRESH_BURST_W<4> {
        REFRESH_BURST_W::new(self)
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    #[must_use]
    pub fn refresh_to_x32(&mut self) -> REFRESH_TO_X32_W<12> {
        REFRESH_TO_X32_W::new(self)
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    #[must_use]
    pub fn refresh_margin(&mut self) -> REFRESH_MARGIN_W<20> {
        REFRESH_MARGIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL refresh control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_rfshctl0](index.html) module"]
pub struct DDRCTRL_RFSHCTL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_RFSHCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_rfshctl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_rfshctl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_RFSHCTL0 to value 0x0021_0000"]
impl crate::Resettable for DDRCTRL_RFSHCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0021_0000;
}
