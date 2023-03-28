#[doc = "Register `CNT_ALTERNATE5` reader"]
pub struct R(crate::R<CNT_ALTERNATE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_ALTERNATE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_ALTERNATE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_ALTERNATE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_ALTERNATE5` writer"]
pub struct W(crate::W<CNT_ALTERNATE5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_ALTERNATE5_SPEC>;
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
impl From<crate::W<CNT_ALTERNATE5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_ALTERNATE5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
pub type CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT` writer - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_ALTERNATE5_SPEC, u32, u32, 31, O>;
#[doc = "Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub type UIFCPY_R = crate::BitReader<bool>;
#[doc = "Field `UIFCPY` writer - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub type UIFCPY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNT_ALTERNATE5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn uifcpy(&mut self) -> UIFCPY_W<31> {
        UIFCPY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_alternate5](index.html) module"]
pub struct CNT_ALTERNATE5_SPEC;
impl crate::RegisterSpec for CNT_ALTERNATE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_alternate5::R](R) reader structure"]
impl crate::Readable for CNT_ALTERNATE5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_alternate5::W](W) writer structure"]
impl crate::Writable for CNT_ALTERNATE5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT_ALTERNATE5 to value 0"]
impl crate::Resettable for CNT_ALTERNATE5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
