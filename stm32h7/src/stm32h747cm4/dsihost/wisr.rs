#[doc = "Register `WISR` reader"]
pub struct R(crate::R<WISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WISR` writer"]
pub struct W(crate::W<WISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WISR_SPEC>;
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
impl From<crate::W<WISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEIF` reader - Tearing effect interrupt flag"]
pub type TEIF_R = crate::BitReader<bool>;
#[doc = "Field `TEIF` writer - Tearing effect interrupt flag"]
pub type TEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
#[doc = "Field `ERIF` reader - End of refresh interrupt flag"]
pub type ERIF_R = crate::BitReader<bool>;
#[doc = "Field `ERIF` writer - End of refresh interrupt flag"]
pub type ERIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - Busy flag"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - Busy flag"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
#[doc = "Field `PLLLS` reader - PLL lock status"]
pub type PLLLS_R = crate::BitReader<bool>;
#[doc = "Field `PLLLS` writer - PLL lock status"]
pub type PLLLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
#[doc = "Field `PLLLIF` reader - PLL lock interrupt flag"]
pub type PLLLIF_R = crate::BitReader<bool>;
#[doc = "Field `PLLLIF` writer - PLL lock interrupt flag"]
pub type PLLLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
#[doc = "Field `PLLUIF` reader - PLL unlock interrupt flag"]
pub type PLLUIF_R = crate::BitReader<bool>;
#[doc = "Field `PLLUIF` writer - PLL unlock interrupt flag"]
pub type PLLUIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
#[doc = "Field `RRS` reader - Regulator ready status"]
pub type RRS_R = crate::BitReader<bool>;
#[doc = "Field `RRS` writer - Regulator ready status"]
pub type RRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
#[doc = "Field `RRIF` reader - Regulator ready interrupt flag"]
pub type RRIF_R = crate::BitReader<bool>;
#[doc = "Field `RRIF` writer - Regulator ready interrupt flag"]
pub type RRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Tearing effect interrupt flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt flag"]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL lock status"]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL lock interrupt flag"]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL unlock interrupt flag"]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Regulator ready status"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Regulator ready interrupt flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tearing effect interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn teif(&mut self) -> TEIF_W<0> {
        TEIF_W::new(self)
    }
    #[doc = "Bit 1 - End of refresh interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn erif(&mut self) -> ERIF_W<1> {
        ERIF_W::new(self)
    }
    #[doc = "Bit 2 - Busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<2> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 8 - PLL lock status"]
    #[inline(always)]
    #[must_use]
    pub fn pllls(&mut self) -> PLLLS_W<8> {
        PLLLS_W::new(self)
    }
    #[doc = "Bit 9 - PLL lock interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn plllif(&mut self) -> PLLLIF_W<9> {
        PLLLIF_W::new(self)
    }
    #[doc = "Bit 10 - PLL unlock interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn plluif(&mut self) -> PLLUIF_W<10> {
        PLLUIF_W::new(self)
    }
    #[doc = "Bit 12 - Regulator ready status"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<12> {
        RRS_W::new(self)
    }
    #[doc = "Bit 13 - Regulator ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rrif(&mut self) -> RRIF_W<13> {
        RRIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wisr](index.html) module"]
pub struct WISR_SPEC;
impl crate::RegisterSpec for WISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wisr::R](R) reader structure"]
impl crate::Readable for WISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wisr::W](W) writer structure"]
impl crate::Writable for WISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WISR to value 0"]
impl crate::Resettable for WISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
