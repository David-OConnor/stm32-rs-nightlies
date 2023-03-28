#[doc = "Register `TAMP_OR` reader"]
pub struct R(crate::R<TAMP_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_OR` writer"]
pub struct W(crate::W<TAMP_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_OR_SPEC>;
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
impl From<crate::W<TAMP_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT3_RMP` reader - TAMP_OUT3 mapping"]
pub type OUT3_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT3_RMP` writer - TAMP_OUT3 mapping"]
pub type OUT3_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_OR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OUT5_RMP` reader - TAMP_OUT5 mapping"]
pub type OUT5_RMP_R = crate::BitReader<bool>;
#[doc = "Field `OUT5_RMP` writer - TAMP_OUT5 mapping"]
pub type OUT5_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_OR_SPEC, bool, O>;
#[doc = "Field `IN2_RMP` reader - TAMP_IN2 mapping"]
pub type IN2_RMP_R = crate::BitReader<bool>;
#[doc = "Field `IN2_RMP` writer - TAMP_IN2 mapping"]
pub type IN2_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_OR_SPEC, bool, O>;
#[doc = "Field `IN3_RMP` reader - TAMP_IN3 mapping"]
pub type IN3_RMP_R = crate::BitReader<bool>;
#[doc = "Field `IN3_RMP` writer - TAMP_IN3 mapping"]
pub type IN3_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_OR_SPEC, bool, O>;
#[doc = "Field `IN4_RMP` reader - TAMP_IN4 mapping"]
pub type IN4_RMP_R = crate::BitReader<bool>;
#[doc = "Field `IN4_RMP` writer - TAMP_IN4 mapping"]
pub type IN4_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_OR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - TAMP_OUT3 mapping"]
    #[inline(always)]
    pub fn out3_rmp(&self) -> OUT3_RMP_R {
        OUT3_RMP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - TAMP_OUT5 mapping"]
    #[inline(always)]
    pub fn out5_rmp(&self) -> OUT5_RMP_R {
        OUT5_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TAMP_IN2 mapping"]
    #[inline(always)]
    pub fn in2_rmp(&self) -> IN2_RMP_R {
        IN2_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TAMP_IN3 mapping"]
    #[inline(always)]
    pub fn in3_rmp(&self) -> IN3_RMP_R {
        IN3_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TAMP_IN4 mapping"]
    #[inline(always)]
    pub fn in4_rmp(&self) -> IN4_RMP_R {
        IN4_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - TAMP_OUT3 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn out3_rmp(&mut self) -> OUT3_RMP_W<1> {
        OUT3_RMP_W::new(self)
    }
    #[doc = "Bit 3 - TAMP_OUT5 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn out5_rmp(&mut self) -> OUT5_RMP_W<3> {
        OUT5_RMP_W::new(self)
    }
    #[doc = "Bit 8 - TAMP_IN2 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn in2_rmp(&mut self) -> IN2_RMP_W<8> {
        IN2_RMP_W::new(self)
    }
    #[doc = "Bit 9 - TAMP_IN3 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn in3_rmp(&mut self) -> IN3_RMP_W<9> {
        IN3_RMP_W::new(self)
    }
    #[doc = "Bit 10 - TAMP_IN4 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn in4_rmp(&mut self) -> IN4_RMP_W<10> {
        IN4_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_or](index.html) module"]
pub struct TAMP_OR_SPEC;
impl crate::RegisterSpec for TAMP_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_or::R](R) reader structure"]
impl crate::Readable for TAMP_OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_or::W](W) writer structure"]
impl crate::Writable for TAMP_OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_OR to value 0"]
impl crate::Resettable for TAMP_OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
