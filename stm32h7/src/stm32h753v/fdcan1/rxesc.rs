#[doc = "Register `RXESC` reader"]
pub struct R(crate::R<RXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXESC` writer"]
pub struct W(crate::W<RXESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXESC_SPEC>;
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
impl From<crate::W<RXESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0DS` reader - Rx FIFO 1 Data Field Size:"]
pub type F0DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0DS` writer - Rx FIFO 1 Data Field Size:"]
pub type F0DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXESC_SPEC, u8, u8, 3, O>;
#[doc = "Field `F1DS` reader - Rx FIFO 0 Data Field Size:"]
pub type F1DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1DS` writer - Rx FIFO 0 Data Field Size:"]
pub type F1DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXESC_SPEC, u8, u8, 3, O>;
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size:"]
pub type RBDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size:"]
pub type RBDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXESC_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0DS_W<0> {
        F0DS_W::new(self)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1DS_W<4> {
        F1DS_W::new(self)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RBDS_W<8> {
        RBDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxesc](index.html) module"]
pub struct RXESC_SPEC;
impl crate::RegisterSpec for RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxesc::R](R) reader structure"]
impl crate::Readable for RXESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxesc::W](W) writer structure"]
impl crate::Writable for RXESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RXESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
