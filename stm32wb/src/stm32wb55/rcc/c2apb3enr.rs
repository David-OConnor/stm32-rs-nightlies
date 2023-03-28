#[doc = "Register `C2APB3ENR` reader"]
pub struct R(crate::R<C2APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB3ENR` writer"]
pub struct W(crate::W<C2APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB3ENR_SPEC>;
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
impl From<crate::W<C2APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEEN` reader - CPU2 BLE interface clock enable"]
pub type BLEEN_R = crate::BitReader<bool>;
#[doc = "Field `BLEEN` writer - CPU2 BLE interface clock enable"]
pub type BLEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB3ENR_SPEC, bool, O>;
#[doc = "Field `EN802` reader - CPU2 802.15.4 interface clock enable"]
pub type EN802_R = crate::BitReader<bool>;
#[doc = "Field `EN802` writer - CPU2 802.15.4 interface clock enable"]
pub type EN802_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB3ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    pub fn bleen(&self) -> BLEEN_R {
        BLEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    pub fn en802(&self) -> EN802_R {
        EN802_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bleen(&mut self) -> BLEEN_W<0> {
        BLEEN_W::new(self)
    }
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn en802(&mut self) -> EN802_W<1> {
        EN802_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB3ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb3enr](index.html) module"]
pub struct C2APB3ENR_SPEC;
impl crate::RegisterSpec for C2APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb3enr::R](R) reader structure"]
impl crate::Readable for C2APB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb3enr::W](W) writer structure"]
impl crate::Writable for C2APB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2APB3ENR to value 0"]
impl crate::Resettable for C2APB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
