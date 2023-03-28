#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITR1_RMP` reader - Internal trigger 1 remap"]
pub type ITR1_RMP_R = crate::BitReader<bool>;
#[doc = "Field `ITR1_RMP` writer - Internal trigger 1 remap"]
pub type ITR1_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, bool, O>;
#[doc = "Field `ETR_RMP` reader - Timer2 ETR remap"]
pub type ETR_RMP_R = crate::BitReader<bool>;
#[doc = "Field `ETR_RMP` writer - Timer2 ETR remap"]
pub type ETR_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, bool, O>;
#[doc = "Field `TI4_RMP` reader - Internal trigger"]
pub type TI4_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI4_RMP` writer - Internal trigger"]
pub type TI4_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Internal trigger 1 remap"]
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer2 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Internal trigger"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal trigger 1 remap"]
    #[inline(always)]
    #[must_use]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W<0> {
        ITR1_RMP_W::new(self)
    }
    #[doc = "Bit 1 - Timer2 ETR remap"]
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<1> {
        ETR_RMP_W::new(self)
    }
    #[doc = "Bits 2:3 - Internal trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<2> {
        TI4_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
