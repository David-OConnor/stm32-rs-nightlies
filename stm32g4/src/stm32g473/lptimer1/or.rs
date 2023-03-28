#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN1` reader - IN1"]
pub type IN1_R = crate::BitReader<bool>;
#[doc = "Field `IN1` writer - IN1"]
pub type IN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
#[doc = "Field `IN2` reader - IN2"]
pub type IN2_R = crate::BitReader<bool>;
#[doc = "Field `IN2` writer - IN2"]
pub type IN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
#[doc = "Field `IN1_2_1` reader - IN1_2_1"]
pub type IN1_2_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN1_2_1` writer - IN1_2_1"]
pub type IN1_2_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IN2_2_1` reader - IN2_2_1"]
pub type IN2_2_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN2_2_1` writer - IN2_2_1"]
pub type IN2_2_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - IN1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - IN1_2_1"]
    #[inline(always)]
    pub fn in1_2_1(&self) -> IN1_2_1_R {
        IN1_2_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - IN2_2_1"]
    #[inline(always)]
    pub fn in2_2_1(&self) -> IN2_2_1_R {
        IN2_2_1_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IN1"]
    #[inline(always)]
    #[must_use]
    pub fn in1(&mut self) -> IN1_W<0> {
        IN1_W::new(self)
    }
    #[doc = "Bit 1 - IN2"]
    #[inline(always)]
    #[must_use]
    pub fn in2(&mut self) -> IN2_W<1> {
        IN2_W::new(self)
    }
    #[doc = "Bits 2:3 - IN1_2_1"]
    #[inline(always)]
    #[must_use]
    pub fn in1_2_1(&mut self) -> IN1_2_1_W<2> {
        IN1_2_1_W::new(self)
    }
    #[doc = "Bits 4:5 - IN2_2_1"]
    #[inline(always)]
    #[must_use]
    pub fn in2_2_1(&mut self) -> IN2_2_1_W<4> {
        IN2_2_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
