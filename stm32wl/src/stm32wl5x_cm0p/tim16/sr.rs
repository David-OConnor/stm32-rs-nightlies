#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UIF_R = crate::BitReader<UIF_A>;
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIF_A {
    #[doc = "0: No update occurred"]
    Clear = 0,
    #[doc = "1: Update interrupt pending."]
    UpdatePending = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
impl UIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::Clear,
            true => UIF_A::UpdatePending,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UIF_A::Clear
    }
    #[doc = "Checks if the value of the field is `UpdatePending`"]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIF_A::UpdatePending
    }
}
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, UIF_A, O>;
impl<'a, const O: u8> UIF_W<'a, O> {
    #[doc = "No update occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIF_A::Clear)
    }
    #[doc = "Update interrupt pending."]
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UIF_A::UpdatePending)
    }
}
#[doc = "Field `CC1IF` reader - Capture/Compare 1 interrupt flag"]
pub type CC1IF_R = crate::BitReader<CC1IFR_A>;
#[doc = "Capture/Compare 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFR_A {
    #[doc = "0: No campture/compare has been detected"]
    NoMatch = 0,
    #[doc = "1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register"]
    Match = 1,
}
impl From<CC1IFR_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1IFR_A {
        match self.bits {
            false => CC1IFR_A::NoMatch,
            true => CC1IFR_A::Match,
        }
    }
    #[doc = "Checks if the value of the field is `NoMatch`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CC1IFR_A::NoMatch
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CC1IFR_A::Match
    }
}
#[doc = "Capture/Compare 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1IFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1IFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` writer - Capture/Compare 1 interrupt flag"]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, CC1IFW_AW, O>;
impl<'a, const O: u8> CC1IF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1IFW_AW::Clear)
    }
}
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type COMIF_R = crate::BitReader<COMIFR_A>;
#[doc = "COM interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIFR_A {
    #[doc = "0: No COM event occurred"]
    NoCom = 0,
    #[doc = "1: COM interrupt pending"]
    Com = 1,
}
impl From<COMIFR_A> for bool {
    #[inline(always)]
    fn from(variant: COMIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMIFR_A {
        match self.bits {
            false => COMIFR_A::NoCom,
            true => COMIFR_A::Com,
        }
    }
    #[doc = "Checks if the value of the field is `NoCom`"]
    #[inline(always)]
    pub fn is_no_com(&self) -> bool {
        *self == COMIFR_A::NoCom
    }
    #[doc = "Checks if the value of the field is `Com`"]
    #[inline(always)]
    pub fn is_com(&self) -> bool {
        *self == COMIFR_A::Com
    }
}
#[doc = "COM interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<COMIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: COMIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type COMIF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, COMIFW_AW, O>;
impl<'a, const O: u8> COMIF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMIFW_AW::Clear)
    }
}
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BIF_R = crate::BitReader<BIFR_A>;
#[doc = "Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIFR_A {
    #[doc = "0: No break event occurred"]
    NoBreak = 0,
    #[doc = "1: Break interrupt pending"]
    Break = 1,
}
impl From<BIFR_A> for bool {
    #[inline(always)]
    fn from(variant: BIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl BIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIFR_A {
        match self.bits {
            false => BIFR_A::NoBreak,
            true => BIFR_A::Break,
        }
    }
    #[doc = "Checks if the value of the field is `NoBreak`"]
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == BIFR_A::NoBreak
    }
    #[doc = "Checks if the value of the field is `Break`"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == BIFR_A::Break
    }
}
#[doc = "Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<BIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: BIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BIF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, BIFW_AW, O>;
impl<'a, const O: u8> BIF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BIFW_AW::Clear)
    }
}
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag"]
pub type CC1OF_R = crate::BitReader<CC1OFR_A>;
#[doc = "Capture/Compare 1 overcapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFR_A {
    #[doc = "0: No overcapture has been detected"]
    NoOvercapture = 0,
    #[doc = "1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
    Overcapture = 1,
}
impl From<CC1OFR_A> for bool {
    #[inline(always)]
    fn from(variant: CC1OFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1OFR_A {
        match self.bits {
            false => CC1OFR_A::NoOvercapture,
            true => CC1OFR_A::Overcapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoOvercapture`"]
    #[inline(always)]
    pub fn is_no_overcapture(&self) -> bool {
        *self == CC1OFR_A::NoOvercapture
    }
    #[doc = "Checks if the value of the field is `Overcapture`"]
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        *self == CC1OFR_A::Overcapture
    }
}
#[doc = "Capture/Compare 1 overcapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1OFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1OFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag"]
pub type CC1OF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, CC1OFW_AW, O>;
impl<'a, const O: u8> CC1OF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1OFW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<1> {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<5> {
        COMIF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<7> {
        BIF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<9> {
        CC1OF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x02a0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
