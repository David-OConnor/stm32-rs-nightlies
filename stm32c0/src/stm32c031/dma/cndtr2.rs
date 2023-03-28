#[doc = "Register `CNDTR2` reader"]
pub struct R(crate::R<CNDTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDTR2` writer"]
pub struct W(crate::W<CNDTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR2_SPEC>;
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
impl From<crate::W<CNDTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type NDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDT` writer - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 2 number of data to transfer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr2](index.html) module"]
pub struct CNDTR2_SPEC;
impl crate::RegisterSpec for CNDTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndtr2::R](R) reader structure"]
impl crate::Readable for CNDTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndtr2::W](W) writer structure"]
impl crate::Writable for CNDTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNDTR2 to value 0"]
impl crate::Resettable for CNDTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
