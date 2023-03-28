#[doc = "Register `SWPR` reader"]
pub struct R(crate::R<SWPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWPR` writer"]
pub struct W(crate::W<SWPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPR_SPEC>;
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
impl From<crate::W<SWPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Page0_WP` reader - Write protection"]
pub type PAGE0_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page0_WP` writer - Write protection"]
pub type PAGE0_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page1_WP` reader - Write protection"]
pub type PAGE1_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page1_WP` writer - Write protection"]
pub type PAGE1_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page2_WP` reader - Write protection"]
pub type PAGE2_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page2_WP` writer - Write protection"]
pub type PAGE2_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page3_WP` reader - Write protection"]
pub type PAGE3_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page3_WP` writer - Write protection"]
pub type PAGE3_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page4_WP` reader - Write protection"]
pub type PAGE4_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page4_WP` writer - Write protection"]
pub type PAGE4_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page5_WP` reader - Write protection"]
pub type PAGE5_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page5_WP` writer - Write protection"]
pub type PAGE5_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page6_WP` reader - Write protection"]
pub type PAGE6_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page6_WP` writer - Write protection"]
pub type PAGE6_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page7_WP` reader - Write protection"]
pub type PAGE7_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page7_WP` writer - Write protection"]
pub type PAGE7_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page8_WP` reader - Write protection"]
pub type PAGE8_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page8_WP` writer - Write protection"]
pub type PAGE8_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page9_WP` reader - Write protection"]
pub type PAGE9_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page9_WP` writer - Write protection"]
pub type PAGE9_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page10_WP` reader - Write protection"]
pub type PAGE10_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page10_WP` writer - Write protection"]
pub type PAGE10_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page11_WP` reader - Write protection"]
pub type PAGE11_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page11_WP` writer - Write protection"]
pub type PAGE11_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page12_WP` reader - Write protection"]
pub type PAGE12_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page12_WP` writer - Write protection"]
pub type PAGE12_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page13_WP` reader - Write protection"]
pub type PAGE13_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page13_WP` writer - Write protection"]
pub type PAGE13_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page14_WP` reader - Write protection"]
pub type PAGE14_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page14_WP` writer - Write protection"]
pub type PAGE14_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page15_WP` reader - Write protection"]
pub type PAGE15_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page15_WP` writer - Write protection"]
pub type PAGE15_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page16_WP` reader - Write protection"]
pub type PAGE16_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page16_WP` writer - Write protection"]
pub type PAGE16_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page17_WP` reader - Write protection"]
pub type PAGE17_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page17_WP` writer - Write protection"]
pub type PAGE17_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page18_WP` reader - Write protection"]
pub type PAGE18_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page18_WP` writer - Write protection"]
pub type PAGE18_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page19_WP` reader - Write protection"]
pub type PAGE19_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page19_WP` writer - Write protection"]
pub type PAGE19_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page20_WP` reader - Write protection"]
pub type PAGE20_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page20_WP` writer - Write protection"]
pub type PAGE20_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page21_WP` reader - Write protection"]
pub type PAGE21_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page21_WP` writer - Write protection"]
pub type PAGE21_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page22_WP` reader - Write protection"]
pub type PAGE22_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page22_WP` writer - Write protection"]
pub type PAGE22_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page23_WP` reader - Write protection"]
pub type PAGE23_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page23_WP` writer - Write protection"]
pub type PAGE23_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page24_WP` reader - Write protection"]
pub type PAGE24_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page24_WP` writer - Write protection"]
pub type PAGE24_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page25_WP` reader - Write protection"]
pub type PAGE25_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page25_WP` writer - Write protection"]
pub type PAGE25_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page26_WP` reader - Write protection"]
pub type PAGE26_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page26_WP` writer - Write protection"]
pub type PAGE26_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page27_WP` reader - Write protection"]
pub type PAGE27_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page27_WP` writer - Write protection"]
pub type PAGE27_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page28_WP` reader - Write protection"]
pub type PAGE28_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page28_WP` writer - Write protection"]
pub type PAGE28_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page29_WP` reader - Write protection"]
pub type PAGE29_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page29_WP` writer - Write protection"]
pub type PAGE29_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page30_WP` reader - Write protection"]
pub type PAGE30_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page30_WP` writer - Write protection"]
pub type PAGE30_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
#[doc = "Field `Page31_WP` reader - Write protection"]
pub type PAGE31_WP_R = crate::BitReader<bool>;
#[doc = "Field `Page31_WP` writer - Write protection"]
pub type PAGE31_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write protection"]
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write protection"]
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write protection"]
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection"]
    #[inline(always)]
    pub fn page4_wp(&self) -> PAGE4_WP_R {
        PAGE4_WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write protection"]
    #[inline(always)]
    pub fn page5_wp(&self) -> PAGE5_WP_R {
        PAGE5_WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write protection"]
    #[inline(always)]
    pub fn page6_wp(&self) -> PAGE6_WP_R {
        PAGE6_WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write protection"]
    #[inline(always)]
    pub fn page7_wp(&self) -> PAGE7_WP_R {
        PAGE7_WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write protection"]
    #[inline(always)]
    pub fn page8_wp(&self) -> PAGE8_WP_R {
        PAGE8_WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn page9_wp(&self) -> PAGE9_WP_R {
        PAGE9_WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write protection"]
    #[inline(always)]
    pub fn page10_wp(&self) -> PAGE10_WP_R {
        PAGE10_WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write protection"]
    #[inline(always)]
    pub fn page11_wp(&self) -> PAGE11_WP_R {
        PAGE11_WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write protection"]
    #[inline(always)]
    pub fn page12_wp(&self) -> PAGE12_WP_R {
        PAGE12_WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write protection"]
    #[inline(always)]
    pub fn page13_wp(&self) -> PAGE13_WP_R {
        PAGE13_WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write protection"]
    #[inline(always)]
    pub fn page14_wp(&self) -> PAGE14_WP_R {
        PAGE14_WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write protection"]
    #[inline(always)]
    pub fn page15_wp(&self) -> PAGE15_WP_R {
        PAGE15_WP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write protection"]
    #[inline(always)]
    pub fn page16_wp(&self) -> PAGE16_WP_R {
        PAGE16_WP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write protection"]
    #[inline(always)]
    pub fn page17_wp(&self) -> PAGE17_WP_R {
        PAGE17_WP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write protection"]
    #[inline(always)]
    pub fn page18_wp(&self) -> PAGE18_WP_R {
        PAGE18_WP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write protection"]
    #[inline(always)]
    pub fn page19_wp(&self) -> PAGE19_WP_R {
        PAGE19_WP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write protection"]
    #[inline(always)]
    pub fn page20_wp(&self) -> PAGE20_WP_R {
        PAGE20_WP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protection"]
    #[inline(always)]
    pub fn page21_wp(&self) -> PAGE21_WP_R {
        PAGE21_WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write protection"]
    #[inline(always)]
    pub fn page22_wp(&self) -> PAGE22_WP_R {
        PAGE22_WP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write protection"]
    #[inline(always)]
    pub fn page23_wp(&self) -> PAGE23_WP_R {
        PAGE23_WP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Write protection"]
    #[inline(always)]
    pub fn page24_wp(&self) -> PAGE24_WP_R {
        PAGE24_WP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write protection"]
    #[inline(always)]
    pub fn page25_wp(&self) -> PAGE25_WP_R {
        PAGE25_WP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write protection"]
    #[inline(always)]
    pub fn page26_wp(&self) -> PAGE26_WP_R {
        PAGE26_WP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write protection"]
    #[inline(always)]
    pub fn page27_wp(&self) -> PAGE27_WP_R {
        PAGE27_WP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Write protection"]
    #[inline(always)]
    pub fn page28_wp(&self) -> PAGE28_WP_R {
        PAGE28_WP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write protection"]
    #[inline(always)]
    pub fn page29_wp(&self) -> PAGE29_WP_R {
        PAGE29_WP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write protection"]
    #[inline(always)]
    pub fn page30_wp(&self) -> PAGE30_WP_R {
        PAGE30_WP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write protection"]
    #[inline(always)]
    pub fn page31_wp(&self) -> PAGE31_WP_R {
        PAGE31_WP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W<0> {
        PAGE0_WP_W::new(self)
    }
    #[doc = "Bit 1 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W<1> {
        PAGE1_WP_W::new(self)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W<2> {
        PAGE2_WP_W::new(self)
    }
    #[doc = "Bit 3 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W<3> {
        PAGE3_WP_W::new(self)
    }
    #[doc = "Bit 4 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page4_wp(&mut self) -> PAGE4_WP_W<4> {
        PAGE4_WP_W::new(self)
    }
    #[doc = "Bit 5 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page5_wp(&mut self) -> PAGE5_WP_W<5> {
        PAGE5_WP_W::new(self)
    }
    #[doc = "Bit 6 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page6_wp(&mut self) -> PAGE6_WP_W<6> {
        PAGE6_WP_W::new(self)
    }
    #[doc = "Bit 7 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page7_wp(&mut self) -> PAGE7_WP_W<7> {
        PAGE7_WP_W::new(self)
    }
    #[doc = "Bit 8 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page8_wp(&mut self) -> PAGE8_WP_W<8> {
        PAGE8_WP_W::new(self)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page9_wp(&mut self) -> PAGE9_WP_W<9> {
        PAGE9_WP_W::new(self)
    }
    #[doc = "Bit 10 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page10_wp(&mut self) -> PAGE10_WP_W<10> {
        PAGE10_WP_W::new(self)
    }
    #[doc = "Bit 11 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page11_wp(&mut self) -> PAGE11_WP_W<11> {
        PAGE11_WP_W::new(self)
    }
    #[doc = "Bit 12 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page12_wp(&mut self) -> PAGE12_WP_W<12> {
        PAGE12_WP_W::new(self)
    }
    #[doc = "Bit 13 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page13_wp(&mut self) -> PAGE13_WP_W<13> {
        PAGE13_WP_W::new(self)
    }
    #[doc = "Bit 14 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page14_wp(&mut self) -> PAGE14_WP_W<14> {
        PAGE14_WP_W::new(self)
    }
    #[doc = "Bit 15 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page15_wp(&mut self) -> PAGE15_WP_W<15> {
        PAGE15_WP_W::new(self)
    }
    #[doc = "Bit 16 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page16_wp(&mut self) -> PAGE16_WP_W<16> {
        PAGE16_WP_W::new(self)
    }
    #[doc = "Bit 17 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page17_wp(&mut self) -> PAGE17_WP_W<17> {
        PAGE17_WP_W::new(self)
    }
    #[doc = "Bit 18 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page18_wp(&mut self) -> PAGE18_WP_W<18> {
        PAGE18_WP_W::new(self)
    }
    #[doc = "Bit 19 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page19_wp(&mut self) -> PAGE19_WP_W<19> {
        PAGE19_WP_W::new(self)
    }
    #[doc = "Bit 20 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page20_wp(&mut self) -> PAGE20_WP_W<20> {
        PAGE20_WP_W::new(self)
    }
    #[doc = "Bit 21 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page21_wp(&mut self) -> PAGE21_WP_W<21> {
        PAGE21_WP_W::new(self)
    }
    #[doc = "Bit 22 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page22_wp(&mut self) -> PAGE22_WP_W<22> {
        PAGE22_WP_W::new(self)
    }
    #[doc = "Bit 23 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page23_wp(&mut self) -> PAGE23_WP_W<23> {
        PAGE23_WP_W::new(self)
    }
    #[doc = "Bit 24 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page24_wp(&mut self) -> PAGE24_WP_W<24> {
        PAGE24_WP_W::new(self)
    }
    #[doc = "Bit 25 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page25_wp(&mut self) -> PAGE25_WP_W<25> {
        PAGE25_WP_W::new(self)
    }
    #[doc = "Bit 26 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page26_wp(&mut self) -> PAGE26_WP_W<26> {
        PAGE26_WP_W::new(self)
    }
    #[doc = "Bit 27 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page27_wp(&mut self) -> PAGE27_WP_W<27> {
        PAGE27_WP_W::new(self)
    }
    #[doc = "Bit 28 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page28_wp(&mut self) -> PAGE28_WP_W<28> {
        PAGE28_WP_W::new(self)
    }
    #[doc = "Bit 29 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page29_wp(&mut self) -> PAGE29_WP_W<29> {
        PAGE29_WP_W::new(self)
    }
    #[doc = "Bit 30 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page30_wp(&mut self) -> PAGE30_WP_W<30> {
        PAGE30_WP_W::new(self)
    }
    #[doc = "Bit 31 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn page31_wp(&mut self) -> PAGE31_WP_W<31> {
        PAGE31_WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Write protection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr](index.html) module"]
pub struct SWPR_SPEC;
impl crate::RegisterSpec for SWPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swpr::R](R) reader structure"]
impl crate::Readable for SWPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swpr::W](W) writer structure"]
impl crate::Writable for SWPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWPR to value 0"]
impl crate::Resettable for SWPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
