#[doc = "Register `C2EMR1` reader"]
pub struct R(crate::R<C2EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2EMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2EMR1` writer"]
pub struct W(crate::W<C2EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2EMR1_SPEC>;
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
impl From<crate::W<C2EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2EMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0_15` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM0_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EM0_15` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM0_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2EMR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `EM17_21` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM17_21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EM17_21` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM17_21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2EMR1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0_15(&self) -> EM0_15_R {
        EM0_15_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17_21(&self) -> EM17_21_R {
        EM17_21_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em0_15(&mut self) -> EM0_15_W<0> {
        EM0_15_W::new(self)
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em17_21(&mut self) -> EM17_21_W<17> {
        EM17_21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr1](index.html) module"]
pub struct C2EMR1_SPEC;
impl crate::RegisterSpec for C2EMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2emr1::R](R) reader structure"]
impl crate::Readable for C2EMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2emr1::W](W) writer structure"]
impl crate::Writable for C2EMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2EMR1 to value 0"]
impl crate::Resettable for C2EMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
