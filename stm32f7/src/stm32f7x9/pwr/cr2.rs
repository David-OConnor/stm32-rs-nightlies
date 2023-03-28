#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWUPF1` reader - Clear Wakeup Pin flag for PA0"]
pub type CWUPF1_R = crate::BitReader<bool>;
#[doc = "Field `CWUPF2` reader - Clear Wakeup Pin flag for PA2"]
pub type CWUPF2_R = crate::BitReader<bool>;
#[doc = "Field `CWUPF3` reader - Clear Wakeup Pin flag for PC1"]
pub type CWUPF3_R = crate::BitReader<bool>;
#[doc = "Field `CWUPF4` reader - Clear Wakeup Pin flag for PC13"]
pub type CWUPF4_R = crate::BitReader<bool>;
#[doc = "Field `CWUPF5` reader - Clear Wakeup Pin flag for PI8"]
pub type CWUPF5_R = crate::BitReader<bool>;
#[doc = "Field `CWUPF6` reader - Clear Wakeup Pin flag for PI11"]
pub type CWUPF6_R = crate::BitReader<bool>;
#[doc = "Field `WUPP1` reader - Wakeup pin polarity bit for PA0"]
pub type WUPP1_R = crate::BitReader<bool>;
#[doc = "Field `WUPP1` writer - Wakeup pin polarity bit for PA0"]
pub type WUPP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `WUPP2` reader - Wakeup pin polarity bit for PA2"]
pub type WUPP2_R = crate::BitReader<bool>;
#[doc = "Field `WUPP2` writer - Wakeup pin polarity bit for PA2"]
pub type WUPP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `WUPP3` reader - Wakeup pin polarity bit for PC1"]
pub type WUPP3_R = crate::BitReader<bool>;
#[doc = "Field `WUPP3` writer - Wakeup pin polarity bit for PC1"]
pub type WUPP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `WUPP4` reader - Wakeup pin polarity bit for PC13"]
pub type WUPP4_R = crate::BitReader<bool>;
#[doc = "Field `WUPP4` writer - Wakeup pin polarity bit for PC13"]
pub type WUPP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `WUPP5` reader - Wakeup pin polarity bit for PI8"]
pub type WUPP5_R = crate::BitReader<bool>;
#[doc = "Field `WUPP5` writer - Wakeup pin polarity bit for PI8"]
pub type WUPP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `WUPP6` reader - Wakeup pin polarity bit for PI11"]
pub type WUPP6_R = crate::BitReader<bool>;
#[doc = "Field `WUPP6` writer - Wakeup pin polarity bit for PI11"]
pub type WUPP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clear Wakeup Pin flag for PA0"]
    #[inline(always)]
    pub fn cwupf1(&self) -> CWUPF1_R {
        CWUPF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear Wakeup Pin flag for PA2"]
    #[inline(always)]
    pub fn cwupf2(&self) -> CWUPF2_R {
        CWUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Wakeup Pin flag for PC1"]
    #[inline(always)]
    pub fn cwupf3(&self) -> CWUPF3_R {
        CWUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Wakeup Pin flag for PC13"]
    #[inline(always)]
    pub fn cwupf4(&self) -> CWUPF4_R {
        CWUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear Wakeup Pin flag for PI8"]
    #[inline(always)]
    pub fn cwupf5(&self) -> CWUPF5_R {
        CWUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear Wakeup Pin flag for PI11"]
    #[inline(always)]
    pub fn cwupf6(&self) -> CWUPF6_R {
        CWUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    #[must_use]
    pub fn wupp1(&mut self) -> WUPP1_W<8> {
        WUPP1_W::new(self)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    #[must_use]
    pub fn wupp2(&mut self) -> WUPP2_W<9> {
        WUPP2_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    #[must_use]
    pub fn wupp3(&mut self) -> WUPP3_W<10> {
        WUPP3_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    #[must_use]
    pub fn wupp4(&mut self) -> WUPP4_W<11> {
        WUPP4_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    #[must_use]
    pub fn wupp5(&mut self) -> WUPP5_W<12> {
        WUPP5_W::new(self)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    #[must_use]
    pub fn wupp6(&mut self) -> WUPP6_W<13> {
        WUPP6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
