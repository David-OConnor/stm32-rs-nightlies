#[doc = "Register `OPFCCR` reader"]
pub struct R(crate::R<OPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPFCCR` writer"]
pub struct W(crate::W<OPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPFCCR_SPEC>;
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
impl From<crate::W<OPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM` reader - Color mode"]
pub type CM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CM` writer - Color mode"]
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPFCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SB` reader - Swap Bytes"]
pub type SB_R = crate::BitReader<bool>;
#[doc = "Field `SB` writer - Swap Bytes"]
pub type SB_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, bool, O>;
#[doc = "Field `AI` reader - Alpha Inverted"]
pub type AI_R = crate::BitReader<bool>;
#[doc = "Field `AI` writer - Alpha Inverted"]
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, bool, O>;
#[doc = "Field `RBS` reader - Red Blue Swap"]
pub type RBS_R = crate::BitReader<bool>;
#[doc = "Field `RBS` writer - Red Blue Swap"]
pub type RBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 9 - Swap Bytes"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 20 - Alpha Inverted"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Red Blue Swap"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    #[doc = "Bit 9 - Swap Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SB_W<9> {
        SB_W::new(self)
    }
    #[doc = "Bit 20 - Alpha Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<20> {
        AI_W::new(self)
    }
    #[doc = "Bit 21 - Red Blue Swap"]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<21> {
        RBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "output PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opfccr](index.html) module"]
pub struct OPFCCR_SPEC;
impl crate::RegisterSpec for OPFCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opfccr::R](R) reader structure"]
impl crate::Readable for OPFCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opfccr::W](W) writer structure"]
impl crate::Writable for OPFCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPFCCR to value 0"]
impl crate::Resettable for OPFCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
