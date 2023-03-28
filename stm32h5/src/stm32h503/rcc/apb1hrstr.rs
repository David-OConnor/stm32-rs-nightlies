#[doc = "Register `APB1HRSTR` reader"]
pub struct R(crate::R<APB1HRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HRSTR` writer"]
pub struct W(crate::W<APB1HRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HRSTR_SPEC>;
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
impl From<crate::W<APB1HRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTSRST` reader - DTS block reset Set and reset by software."]
pub type DTSRST_R = crate::BitReader<bool>;
#[doc = "Field `DTSRST` writer - DTS block reset Set and reset by software."]
pub type DTSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HRSTR_SPEC, bool, O>;
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset Set and reset by software."]
pub type LPTIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset Set and reset by software."]
pub type LPTIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HRSTR_SPEC, bool, O>;
#[doc = "Field `FDCAN1RST` reader - FDCAN1 block reset Set and reset by software."]
pub type FDCAN1RST_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1RST` writer - FDCAN1 block reset Set and reset by software."]
pub type FDCAN1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HRSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - DTS block reset Set and reset by software."]
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn fdcan1rst(&self) -> FDCAN1RST_R {
        FDCAN1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DTS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dtsrst(&mut self) -> DTSRST_W<3> {
        DTSRST_W::new(self)
    }
    #[doc = "Bit 5 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<5> {
        LPTIM2RST_W::new(self)
    }
    #[doc = "Bit 9 - FDCAN1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1rst(&mut self) -> FDCAN1RST_W<9> {
        FDCAN1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 peripheral high reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hrstr](index.html) module"]
pub struct APB1HRSTR_SPEC;
impl crate::RegisterSpec for APB1HRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1hrstr::R](R) reader structure"]
impl crate::Readable for APB1HRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1hrstr::W](W) writer structure"]
impl crate::Writable for APB1HRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1HRSTR to value 0"]
impl crate::Resettable for APB1HRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
