#[doc = "Register `DTFR` reader"]
pub struct R(crate::R<DTFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTFR` writer"]
pub struct W(crate::W<DTFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTFR_SPEC>;
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
impl From<crate::W<DTFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub type DTRX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub type DTRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTFR_SPEC, u16, u16, 9, O>;
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub type SDTRX_R = crate::BitReader<bool>;
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub type SDTRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFR_SPEC, bool, O>;
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub type DTPRSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub type DTPRSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTFR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub type DTRSLKX_R = crate::BitReader<bool>;
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub type DTRSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFR_SPEC, bool, O>;
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub type DTRLKX_R = crate::BitReader<bool>;
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub type DTRLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFR_SPEC, bool, O>;
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub type DTFX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub type DTFX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTFR_SPEC, u16, u16, 9, O>;
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub type SDTFX_R = crate::BitReader<bool>;
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub type SDTFX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFR_SPEC, bool, O>;
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub type DTFSLKX_R = crate::BitReader<bool>;
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub type DTFSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFR_SPEC, bool, O>;
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub type DTFLKX_R = crate::BitReader<bool>;
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub type DTFLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DTRX_W<0> {
        DTRX_W::new(self)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SDTRX_W<9> {
        SDTRX_W::new(self)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DTPRSC_W<10> {
        DTPRSC_W::new(self)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W<14> {
        DTRSLKX_W::new(self)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DTRLKX_W<15> {
        DTRLKX_W::new(self)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DTFX_W<16> {
        DTFX_W::new(self)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SDTFX_W<25> {
        SDTFX_W::new(self)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W<30> {
        DTFSLKX_W::new(self)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DTFLKX_W<31> {
        DTFLKX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtfr](index.html) module"]
pub struct DTFR_SPEC;
impl crate::RegisterSpec for DTFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtfr::R](R) reader structure"]
impl crate::Readable for DTFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtfr::W](W) writer structure"]
impl crate::Writable for DTFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTFR to value 0"]
impl crate::Resettable for DTFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
