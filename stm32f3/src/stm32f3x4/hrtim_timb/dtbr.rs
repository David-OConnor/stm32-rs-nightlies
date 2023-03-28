#[doc = "Register `DTBR` reader"]
pub struct R(crate::R<DTBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTBR` writer"]
pub struct W(crate::W<DTBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTBR_SPEC>;
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
impl From<crate::W<DTBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub type DTRX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub type DTRX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DTBR_SPEC, u16, u16, 9, O>;
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub type SDTRX_R = crate::BitReader<SDTRX_A>;
#[doc = "Sign Deadtime Rising value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTRX_A {
    #[doc = "0: Positive deadtime on rising edge"]
    Positive = 0,
    #[doc = "1: Negative deadtime on rising edge"]
    Negative = 1,
}
impl From<SDTRX_A> for bool {
    #[inline(always)]
    fn from(variant: SDTRX_A) -> Self {
        variant as u8 != 0
    }
}
impl SDTRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDTRX_A {
        match self.bits {
            false => SDTRX_A::Positive,
            true => SDTRX_A::Negative,
        }
    }
    #[doc = "Checks if the value of the field is `Positive`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTRX_A::Positive
    }
    #[doc = "Checks if the value of the field is `Negative`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTRX_A::Negative
    }
}
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub type SDTRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTBR_SPEC, SDTRX_A, O>;
impl<'a, const O: u8> SDTRX_W<'a, O> {
    #[doc = "Positive deadtime on rising edge"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(SDTRX_A::Positive)
    }
    #[doc = "Negative deadtime on rising edge"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(SDTRX_A::Negative)
    }
}
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub type DTPRSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub type DTPRSC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DTBR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub type DTRSLKX_R = crate::BitReader<DTRSLKX_A>;
#[doc = "Deadtime Rising Sign Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRSLKX_A {
    #[doc = "0: Deadtime rising sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime rising sign is read-only"]
    Locked = 1,
}
impl From<DTRSLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTRSLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTRSLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTRSLKX_A {
        match self.bits {
            false => DTRSLKX_A::Unlocked,
            true => DTRSLKX_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRSLKX_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRSLKX_A::Locked
    }
}
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub type DTRSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTBR_SPEC, DTRSLKX_A, O>;
impl<'a, const O: u8> DTRSLKX_W<'a, O> {
    #[doc = "Deadtime rising sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTRSLKX_A::Unlocked)
    }
    #[doc = "Deadtime rising sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTRSLKX_A::Locked)
    }
}
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub type DTRLKX_R = crate::BitReader<DTRLKX_A>;
#[doc = "Deadtime Rising Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRLKX_A {
    #[doc = "0: Deadtime rising value and sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime rising value and sign is read-only"]
    Locked = 1,
}
impl From<DTRLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTRLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTRLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTRLKX_A {
        match self.bits {
            false => DTRLKX_A::Unlocked,
            true => DTRLKX_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRLKX_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRLKX_A::Locked
    }
}
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub type DTRLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTBR_SPEC, DTRLKX_A, O>;
impl<'a, const O: u8> DTRLKX_W<'a, O> {
    #[doc = "Deadtime rising value and sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTRLKX_A::Unlocked)
    }
    #[doc = "Deadtime rising value and sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTRLKX_A::Locked)
    }
}
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub type DTFX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub type DTFX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DTBR_SPEC, u16, u16, 9, O>;
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub type SDTFX_R = crate::BitReader<SDTFX_A>;
#[doc = "Sign Deadtime Falling value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTFX_A {
    #[doc = "0: Positive deadtime on falling edge"]
    Positive = 0,
    #[doc = "1: Negative deadtime on falling edge"]
    Negative = 1,
}
impl From<SDTFX_A> for bool {
    #[inline(always)]
    fn from(variant: SDTFX_A) -> Self {
        variant as u8 != 0
    }
}
impl SDTFX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDTFX_A {
        match self.bits {
            false => SDTFX_A::Positive,
            true => SDTFX_A::Negative,
        }
    }
    #[doc = "Checks if the value of the field is `Positive`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTFX_A::Positive
    }
    #[doc = "Checks if the value of the field is `Negative`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTFX_A::Negative
    }
}
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub type SDTFX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTBR_SPEC, SDTFX_A, O>;
impl<'a, const O: u8> SDTFX_W<'a, O> {
    #[doc = "Positive deadtime on falling edge"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(SDTFX_A::Positive)
    }
    #[doc = "Negative deadtime on falling edge"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(SDTFX_A::Negative)
    }
}
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub type DTFSLKX_R = crate::BitReader<DTFSLKX_A>;
#[doc = "Deadtime Falling Sign Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFSLKX_A {
    #[doc = "0: Deadtime falling sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime falling sign is read-only"]
    Locked = 1,
}
impl From<DTFSLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTFSLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTFSLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFSLKX_A {
        match self.bits {
            false => DTFSLKX_A::Unlocked,
            true => DTFSLKX_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFSLKX_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFSLKX_A::Locked
    }
}
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub type DTFSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTBR_SPEC, DTFSLKX_A, O>;
impl<'a, const O: u8> DTFSLKX_W<'a, O> {
    #[doc = "Deadtime falling sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTFSLKX_A::Unlocked)
    }
    #[doc = "Deadtime falling sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTFSLKX_A::Locked)
    }
}
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub type DTFLKX_R = crate::BitReader<DTFLKX_A>;
#[doc = "Deadtime Falling Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFLKX_A {
    #[doc = "0: Deadtime falling value and sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime falling value and sign is read-only"]
    Locked = 1,
}
impl From<DTFLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTFLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTFLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFLKX_A {
        match self.bits {
            false => DTFLKX_A::Unlocked,
            true => DTFLKX_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFLKX_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFLKX_A::Locked
    }
}
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub type DTFLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTBR_SPEC, DTFLKX_A, O>;
impl<'a, const O: u8> DTFLKX_W<'a, O> {
    #[doc = "Deadtime falling value and sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTFLKX_A::Unlocked)
    }
    #[doc = "Deadtime falling value and sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTFLKX_A::Locked)
    }
}
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
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtbr](index.html) module"]
pub struct DTBR_SPEC;
impl crate::RegisterSpec for DTBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtbr::R](R) reader structure"]
impl crate::Readable for DTBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtbr::W](W) writer structure"]
impl crate::Writable for DTBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTBR to value 0"]
impl crate::Resettable for DTBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
